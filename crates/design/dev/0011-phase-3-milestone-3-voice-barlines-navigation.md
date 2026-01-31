# Phase 3, Milestone 3: Voice, Barlines, Navigation

> **Document Series:** 4 of 7
> **Tasks:** 3.1–3.4
> **Focus:** Multi-voice support, barlines, repeats, voltas

---

## Overview

This milestone adds support for:

- `<backup>` and `<forward>` for multi-voice notation
- `<barline>` with styles, repeats, and endings
- Navigation structures (voltas, D.C., D.S.)

After this milestone, you can parse scores with multiple voices and repeat structures.

---

## Task 3.1: Implement Backup and Forward Parsing

These elements move the time position within a measure, enabling multiple voices.

### Element Structure

```xml
<measure number="1">
  <attributes><divisions>1</divisions></attributes>
  <!-- Voice 1 -->
  <note><pitch><step>C</step><octave>5</octave></pitch><duration>4</duration><voice>1</voice></note>

  <!-- Go back to start of measure for voice 2 -->
  <backup><duration>4</duration></backup>

  <!-- Voice 2 -->
  <note><pitch><step>E</step><octave>4</octave></pitch><duration>4</duration><voice>2</voice></note>
</measure>
```

### Implementation

```rust
fn parse_backup(reader: &mut XmlReader) -> Result<Backup, ParseError> {
    let mut duration = None;

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"duration" => duration = Some(reader.read_text_as()?),
                    // Editorial elements (footnote, level) - skip for now
                    _ => reader.skip_element()?,
                }
            }
            Event::End(e) if e.name().as_ref() == b"backup" => break,
            _ => {}
        }
    }

    Ok(Backup {
        duration: duration.ok_or_else(|| ParseError::MissingElement {
            parent: "backup".to_string(),
            element: "duration".to_string(),
            position: reader.position(),
        })?,
        editorial: None,
    })
}

fn parse_forward(reader: &mut XmlReader) -> Result<Forward, ParseError> {
    let mut duration = None;
    let mut voice = None;
    let mut staff = None;

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"duration" => duration = Some(reader.read_text_as()?),
                    b"voice" => voice = Some(reader.read_text()?),
                    b"staff" => staff = Some(reader.read_text_as()?),
                    // Editorial elements - skip for now
                    _ => reader.skip_element()?,
                }
            }
            Event::End(e) if e.name().as_ref() == b"forward" => break,
            _ => {}
        }
    }

    Ok(Forward {
        duration: duration.ok_or_else(|| ParseError::MissingElement {
            parent: "forward".to_string(),
            element: "duration".to_string(),
            position: reader.position(),
        })?,
        editorial_voice: None,
        voice,
        staff,
    })
}
```

**Acceptance Criteria:**

- [ ] Parse backup with duration
- [ ] Parse forward with duration, optional voice/staff
- [ ] Multi-voice measures parse correctly

---

## Task 3.2: Implement Barline Parsing

### Element Structure

```xml
<!-- Start repeat -->
<barline location="left">
  <bar-style>heavy-light</bar-style>
  <repeat direction="forward"/>
</barline>

<!-- End repeat with volta -->
<barline location="right">
  <bar-style>light-heavy</bar-style>
  <ending number="1" type="stop"/>
  <repeat direction="backward"/>
</barline>

<!-- Second ending start -->
<barline location="left">
  <ending number="2" type="start">2.</ending>
</barline>
```

### Implementation

```rust
fn parse_barline(reader: &mut XmlReader, start: &BytesStart) -> Result<Barline, ParseError> {
    let location = reader.get_optional_attr(start, "location")
        .and_then(|s| parse_right_left_middle(&s).ok());
    let segno = reader.get_optional_attr(start, "segno");
    let coda = reader.get_optional_attr(start, "coda");
    let divisions = reader.get_optional_attr_as(start, "divisions");
    let id = reader.get_optional_attr(start, "id");

    let mut bar_style = None;
    let mut wavy_line = None;
    let mut segno_elem = None;
    let mut coda_elem = None;
    let mut fermatas = Vec::new();
    let mut ending = None;
    let mut repeat = None;

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"bar-style" => bar_style = Some(parse_bar_style_element(reader, &e)?),
                    b"wavy-line" => wavy_line = Some(parse_wavy_line(reader, &e)?),
                    b"segno" => segno_elem = Some(parse_segno(reader, &e)?),
                    b"coda" => coda_elem = Some(parse_coda(reader, &e)?),
                    b"fermata" => fermatas.push(parse_fermata(reader, &e)?),
                    b"ending" => ending = Some(parse_ending(reader, &e)?),
                    b"repeat" => repeat = Some(parse_repeat(reader, &e)?),
                    _ => reader.skip_element()?,
                }
            }
            Event::Empty(e) => {
                match e.name().as_ref() {
                    b"repeat" => repeat = Some(parse_repeat_from_empty(&e)?),
                    b"segno" => segno_elem = Some(parse_segno_from_empty(&e)?),
                    b"coda" => coda_elem = Some(parse_coda_from_empty(&e)?),
                    b"fermata" => fermatas.push(parse_fermata_from_empty(&e)?),
                    _ => {}
                }
            }
            Event::End(e) if e.name().as_ref() == b"barline" => break,
            _ => {}
        }
    }

    Ok(Barline {
        location,
        segno,
        coda,
        divisions,
        bar_style,
        editorial: None,
        wavy_line,
        segno: segno_elem,
        coda: coda_elem,
        fermatas,
        ending,
        repeat,
        id,
    })
}

fn parse_bar_style_element(reader: &mut XmlReader, start: &BytesStart) -> Result<BarStyle, ParseError> {
    let color = reader.get_optional_attr(start, "color");
    let value = parse_bar_style(&reader.read_text()?)?;
    Ok(BarStyle { value, color })
}

fn parse_ending(reader: &mut XmlReader, start: &BytesStart) -> Result<Ending, ParseError> {
    let number = reader.get_attr(start, "number")?;
    let type_str = reader.get_attr(start, "type")?;
    let r#type = parse_start_stop_discontinue(&type_str)?;
    let end_length = reader.get_optional_attr_as(start, "end-length");
    let text_x = reader.get_optional_attr_as(start, "text-x");
    let text_y = reader.get_optional_attr_as(start, "text-y");
    let print_object = reader.get_optional_attr(start, "print-object")
        .and_then(|s| parse_yes_no(&s).ok());

    // Text content is optional
    let text = reader.read_text().ok().filter(|s| !s.is_empty());

    Ok(Ending {
        number,
        r#type,
        text,
        end_length,
        text_x,
        text_y,
        print_object,
        print_style: None,
    })
}

fn parse_repeat(reader: &mut XmlReader, start: &BytesStart) -> Result<Repeat, ParseError> {
    let direction = parse_backward_forward(&reader.get_attr(start, "direction")?)?;
    let times = reader.get_optional_attr_as(start, "times");
    let after_jump = reader.get_optional_attr(start, "after-jump")
        .and_then(|s| parse_yes_no(&s).ok());
    let winged = reader.get_optional_attr(start, "winged"); // TODO: parse Winged enum

    reader.skip_element()?;

    Ok(Repeat {
        direction,
        times,
        after_jump,
        winged: None,
    })
}

fn parse_repeat_from_empty(start: &BytesStart) -> Result<Repeat, ParseError> {
    let direction = parse_backward_forward(&get_attr(start, "direction", 0)?)?;
    let times = get_optional_attr(start, "times").and_then(|s| s.parse().ok());
    let after_jump = get_optional_attr(start, "after-jump")
        .and_then(|s| parse_yes_no(&s).ok());

    Ok(Repeat {
        direction,
        times,
        after_jump,
        winged: None,
    })
}

// Navigation elements
fn parse_segno(reader: &mut XmlReader, start: &BytesStart) -> Result<Segno, ParseError> {
    let id = reader.get_optional_attr(start, "id");
    reader.skip_element()?;
    Ok(Segno {
        print_style: None,
        id,
    })
}

fn parse_segno_from_empty(start: &BytesStart) -> Result<Segno, ParseError> {
    Ok(Segno {
        print_style: None,
        id: get_optional_attr(start, "id"),
    })
}

fn parse_coda(reader: &mut XmlReader, start: &BytesStart) -> Result<Coda, ParseError> {
    let id = reader.get_optional_attr(start, "id");
    reader.skip_element()?;
    Ok(Coda {
        print_style: None,
        smufl: None,
        id,
    })
}

fn parse_coda_from_empty(start: &BytesStart) -> Result<Coda, ParseError> {
    Ok(Coda {
        print_style: None,
        smufl: None,
        id: get_optional_attr(start, "id"),
    })
}

fn parse_wavy_line(reader: &mut XmlReader, start: &BytesStart) -> Result<WavyLine, ParseError> {
    let type_str = reader.get_attr(start, "type")?;
    let r#type = parse_start_stop_continue(&type_str)?;
    let number = reader.get_optional_attr_as(start, "number");

    reader.skip_element()?;

    Ok(WavyLine {
        r#type,
        number,
        default_x: None,
        default_y: None,
        relative_x: None,
        relative_y: None,
        placement: None,
        color: None,
        start_note: None,
        trill_step: None,
        two_note_turn: None,
        accelerate: None,
        beats: None,
        second_beat: None,
        last_beat: None,
    })
}
```

**Acceptance Criteria:**

- [ ] Parse bar styles (regular, light-heavy, heavy-light, etc.)
- [ ] Parse repeat signs (forward, backward, times)
- [ ] Parse endings/voltas (number, type, text)
- [ ] Parse segno and coda elements
- [ ] Handle barline location (left, right, middle)

---

## Task 3.3: Write Multi-Voice Parse Test

### Test Implementation

```rust
#[test]
fn test_parse_two_voices() {
    let xml = r#"<?xml version="1.0"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1"><part-name>Piano</part-name></score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <attributes>
        <divisions>1</divisions>
        <key><fifths>0</fifths></key>
        <time><beats>4</beats><beat-type>4</beat-type></time>
        <clef><sign>G</sign><line>2</line></clef>
      </attributes>

      <!-- Voice 1: C5 whole note -->
      <note>
        <pitch><step>C</step><octave>5</octave></pitch>
        <duration>4</duration>
        <voice>1</voice>
        <type>whole</type>
      </note>

      <!-- Backup to start of measure -->
      <backup><duration>4</duration></backup>

      <!-- Voice 2: E4 whole note -->
      <note>
        <pitch><step>E</step><octave>4</octave></pitch>
        <duration>4</duration>
        <voice>2</voice>
        <type>whole</type>
      </note>
    </measure>
  </part>
</score-partwise>"#;

    let score = parse(xml).expect("should parse two-voice score");

    // Verify structure
    let measure = &score.parts[0].measures[0];

    // Should have: attributes, note(v1), backup, note(v2)
    assert_eq!(measure.content.len(), 4);

    // Check first note is voice 1
    match &measure.content[1] {
        MusicDataElement::Note(n) => {
            assert_eq!(n.voice, Some("1".to_string()));
            match &n.content {
                NoteContent::Regular { full_note, .. } => {
                    match &full_note.content {
                        PitchRestUnpitched::Pitch(p) => {
                            assert_eq!(p.step, Step::C);
                            assert_eq!(p.octave, 5);
                        }
                        _ => panic!("Expected pitch"),
                    }
                }
                _ => panic!("Expected regular note"),
            }
        }
        _ => panic!("Expected note"),
    }

    // Check backup
    match &measure.content[2] {
        MusicDataElement::Backup(b) => {
            assert_eq!(b.duration, 4);
        }
        _ => panic!("Expected backup"),
    }

    // Check second note is voice 2
    match &measure.content[3] {
        MusicDataElement::Note(n) => {
            assert_eq!(n.voice, Some("2".to_string()));
            match &n.content {
                NoteContent::Regular { full_note, .. } => {
                    match &full_note.content {
                        PitchRestUnpitched::Pitch(p) => {
                            assert_eq!(p.step, Step::E);
                            assert_eq!(p.octave, 4);
                        }
                        _ => panic!("Expected pitch"),
                    }
                }
                _ => panic!("Expected regular note"),
            }
        }
        _ => panic!("Expected note"),
    }

    // Round-trip test
    let re_emitted = emit(&score).expect("should emit");
    let re_parsed = parse(&re_emitted).expect("should re-parse");
    assert_eq!(score.parts[0].measures[0].content.len(),
               re_parsed.parts[0].measures[0].content.len());
}

#[test]
fn test_parse_three_voices_with_forward() {
    let xml = r#"<?xml version="1.0"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1"><part-name>Test</part-name></score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <attributes><divisions>4</divisions></attributes>

      <!-- Voice 1: half note -->
      <note>
        <pitch><step>C</step><octave>5</octave></pitch>
        <duration>8</duration>
        <voice>1</voice>
        <type>half</type>
      </note>

      <!-- Forward to skip a quarter -->
      <forward><duration>4</duration></forward>

      <!-- Voice 1: quarter note (fills the measure) -->
      <note>
        <pitch><step>D</step><octave>5</octave></pitch>
        <duration>4</duration>
        <voice>1</voice>
        <type>quarter</type>
      </note>
    </measure>
  </part>
</score-partwise>"#;

    let score = parse(xml).expect("should parse");
    let measure = &score.parts[0].measures[0];

    // Check forward element
    let has_forward = measure.content.iter()
        .any(|e| matches!(e, MusicDataElement::Forward(_)));
    assert!(has_forward, "Should have forward element");
}
```

**Acceptance Criteria:**

- [ ] Parse two-voice measure correctly
- [ ] Backup duration is preserved
- [ ] Voice assignments are preserved
- [ ] Forward element parses correctly

---

## Task 3.4: Write Repeat/Volta Parse Test

### Test Implementation

```rust
#[test]
fn test_parse_simple_repeat() {
    let xml = r#"<?xml version="1.0"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1"><part-name>Test</part-name></score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <attributes><divisions>1</divisions></attributes>
      <barline location="left">
        <bar-style>heavy-light</bar-style>
        <repeat direction="forward"/>
      </barline>
      <note>
        <pitch><step>C</step><octave>4</octave></pitch>
        <duration>4</duration>
        <type>whole</type>
      </note>
    </measure>
    <measure number="2">
      <note>
        <pitch><step>D</step><octave>4</octave></pitch>
        <duration>4</duration>
        <type>whole</type>
      </note>
      <barline location="right">
        <bar-style>light-heavy</bar-style>
        <repeat direction="backward"/>
      </barline>
    </measure>
  </part>
</score-partwise>"#;

    let score = parse(xml).expect("should parse repeat");

    // Check first measure has forward repeat
    let m1 = &score.parts[0].measures[0];
    let barline1 = m1.content.iter()
        .find_map(|e| match e {
            MusicDataElement::Barline(b) => Some(b),
            _ => None,
        })
        .expect("should have barline");

    assert_eq!(barline1.location, Some(RightLeftMiddle::Left));
    assert!(barline1.repeat.is_some());
    assert_eq!(barline1.repeat.as_ref().unwrap().direction, BackwardForward::Forward);

    // Check second measure has backward repeat
    let m2 = &score.parts[0].measures[1];
    let barline2 = m2.content.iter()
        .find_map(|e| match e {
            MusicDataElement::Barline(b) => Some(b),
            _ => None,
        })
        .expect("should have barline");

    assert_eq!(barline2.location, Some(RightLeftMiddle::Right));
    assert!(barline2.repeat.is_some());
    assert_eq!(barline2.repeat.as_ref().unwrap().direction, BackwardForward::Backward);
}

#[test]
fn test_parse_volta_endings() {
    let xml = r#"<?xml version="1.0"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1"><part-name>Test</part-name></score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <attributes><divisions>1</divisions></attributes>
      <barline location="left">
        <repeat direction="forward"/>
      </barline>
      <note>
        <pitch><step>C</step><octave>4</octave></pitch>
        <duration>4</duration>
        <type>whole</type>
      </note>
    </measure>
    <measure number="2">
      <barline location="left">
        <ending number="1" type="start">1.</ending>
      </barline>
      <note>
        <pitch><step>D</step><octave>4</octave></pitch>
        <duration>4</duration>
        <type>whole</type>
      </note>
      <barline location="right">
        <ending number="1" type="stop"/>
        <repeat direction="backward"/>
      </barline>
    </measure>
    <measure number="3">
      <barline location="left">
        <ending number="2" type="start">2.</ending>
      </barline>
      <note>
        <pitch><step>E</step><octave>4</octave></pitch>
        <duration>4</duration>
        <type>whole</type>
      </note>
      <barline location="right">
        <ending number="2" type="discontinue"/>
      </barline>
    </measure>
  </part>
</score-partwise>"#;

    let score = parse(xml).expect("should parse voltas");

    // Check measure 2 has first ending
    let m2 = &score.parts[0].measures[1];
    let barlines: Vec<_> = m2.content.iter()
        .filter_map(|e| match e {
            MusicDataElement::Barline(b) => Some(b),
            _ => None,
        })
        .collect();

    assert_eq!(barlines.len(), 2);

    // Left barline should have ending start
    let left_barline = barlines.iter()
        .find(|b| b.location == Some(RightLeftMiddle::Left))
        .expect("should have left barline");
    assert!(left_barline.ending.is_some());
    let ending = left_barline.ending.as_ref().unwrap();
    assert_eq!(ending.number, "1");
    assert_eq!(ending.r#type, StartStopDiscontinue::Start);
    assert_eq!(ending.text, Some("1.".to_string()));

    // Right barline should have ending stop and repeat
    let right_barline = barlines.iter()
        .find(|b| b.location == Some(RightLeftMiddle::Right))
        .expect("should have right barline");
    assert!(right_barline.ending.is_some());
    assert_eq!(right_barline.ending.as_ref().unwrap().r#type, StartStopDiscontinue::Stop);
    assert!(right_barline.repeat.is_some());

    // Check measure 3 has second ending with discontinue
    let m3 = &score.parts[0].measures[2];
    let right_barline = m3.content.iter()
        .filter_map(|e| match e {
            MusicDataElement::Barline(b) if b.location == Some(RightLeftMiddle::Right) => Some(b),
            _ => None,
        })
        .next()
        .expect("should have right barline");

    assert!(right_barline.ending.is_some());
    assert_eq!(right_barline.ending.as_ref().unwrap().r#type, StartStopDiscontinue::Discontinue);
}

#[test]
fn test_parse_segno_coda() {
    let xml = r#"<?xml version="1.0"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1"><part-name>Test</part-name></score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <attributes><divisions>1</divisions></attributes>
      <barline location="left">
        <segno/>
      </barline>
      <note>
        <pitch><step>C</step><octave>4</octave></pitch>
        <duration>4</duration>
        <type>whole</type>
      </note>
    </measure>
    <measure number="2">
      <note>
        <pitch><step>D</step><octave>4</octave></pitch>
        <duration>4</duration>
        <type>whole</type>
      </note>
      <barline location="right">
        <coda/>
      </barline>
    </measure>
  </part>
</score-partwise>"#;

    let score = parse(xml).expect("should parse segno/coda");

    // Check segno in measure 1
    let m1_barline = score.parts[0].measures[0].content.iter()
        .find_map(|e| match e {
            MusicDataElement::Barline(b) => Some(b),
            _ => None,
        })
        .expect("should have barline");
    assert!(m1_barline.segno.is_some());

    // Check coda in measure 2
    let m2_barline = score.parts[0].measures[1].content.iter()
        .find_map(|e| match e {
            MusicDataElement::Barline(b) => Some(b),
            _ => None,
        })
        .expect("should have barline");
    assert!(m2_barline.coda.is_some());
}
```

**Acceptance Criteria:**

- [ ] Parse forward and backward repeats
- [ ] Parse volta endings (start, stop, discontinue)
- [ ] Parse ending text content
- [ ] Parse segno and coda elements
- [ ] Round-trip preserves all repeat structure

---

## Milestone 3 Summary

After completing Milestone 3:

1. **Backup/Forward** — multi-voice notation support
2. **Barlines** — styles and locations
3. **Repeats** — forward, backward, with times
4. **Endings** — volta brackets with numbers and text
5. **Navigation** — segno and coda markers

You can now parse scores with multiple voices and complex repeat structures.

---

*Next document: Milestone 4 — Directions & Notations*

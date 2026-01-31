# Phase 3, Milestone 2: Core Note & Attributes

> **Document Series:** 3 of 7
> **Tasks:** 2.1–2.5
> **Focus:** Parsing notes, attributes, and the round-trip test

---

## Overview

This milestone implements the core parsing functionality:

- `<attributes>` (key, time, clef, divisions)
- `<note>` (the most complex element in MusicXML)
- Supporting elements (pitch, rest, accidental, beam, etc.)
- The critical round-trip test

After this milestone, you can parse and round-trip simple melodies.

---

## Task 2.1: Implement Attributes Parsing

### Element Structure

```xml
<attributes>
  <divisions>1</divisions>
  <key>
    <fifths>0</fifths>
    <mode>major</mode>
  </key>
  <time>
    <beats>4</beats>
    <beat-type>4</beat-type>
  </time>
  <clef>
    <sign>G</sign>
    <line>2</line>
  </clef>
</attributes>
```

### Implementation

```rust
fn parse_attributes(reader: &mut XmlReader) -> Result<Attributes, ParseError> {
    let mut divisions = None;
    let mut keys = Vec::new();
    let mut times = Vec::new();
    let mut staves = None;
    let mut part_symbol = None;
    let mut instruments = None;
    let mut clefs = Vec::new();
    let mut staff_details = Vec::new();
    let mut transposes = Vec::new();
    let mut for_parts = Vec::new();
    let mut directives = Vec::new();
    let mut measure_styles = Vec::new();

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"divisions" => divisions = Some(reader.read_text_as()?),
                    b"key" => keys.push(parse_key(reader, &e)?),
                    b"time" => times.push(parse_time(reader, &e)?),
                    b"staves" => staves = Some(reader.read_text_as()?),
                    b"part-symbol" => part_symbol = Some(parse_part_symbol(reader, &e)?),
                    b"instruments" => instruments = Some(reader.read_text_as()?),
                    b"clef" => clefs.push(parse_clef(reader, &e)?),
                    b"staff-details" => staff_details.push(parse_staff_details(reader, &e)?),
                    b"transpose" => transposes.push(parse_transpose(reader, &e)?),
                    b"for-part" => for_parts.push(parse_for_part(reader, &e)?),
                    b"directive" => directives.push(parse_directive(reader, &e)?),
                    b"measure-style" => measure_styles.push(parse_measure_style(reader, &e)?),
                    _ => reader.skip_element()?,
                }
            }
            Event::End(e) if e.name().as_ref() == b"attributes" => break,
            _ => {}
        }
    }

    Ok(Attributes {
        divisions,
        keys,
        times,
        staves,
        part_symbol,
        instruments,
        clefs,
        staff_details,
        transposes,
        for_parts,
        directives,
        measure_styles,
    })
}

fn parse_key(reader: &mut XmlReader, start: &BytesStart) -> Result<Key, ParseError> {
    let number = reader.get_optional_attr_as(start, "number");
    let print_object = reader.get_optional_attr(start, "print-object")
        .and_then(|s| parse_yes_no(&s).ok());
    let id = reader.get_optional_attr(start, "id");

    let mut fifths = None;
    let mut mode = None;
    let mut cancel = None;

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"cancel" => cancel = Some(parse_cancel(reader, &e)?),
                    b"fifths" => fifths = Some(reader.read_text_as()?),
                    b"mode" => mode = Some(parse_mode(&reader.read_text()?)?),
                    _ => reader.skip_element()?,
                }
            }
            Event::End(e) if e.name().as_ref() == b"key" => break,
            _ => {}
        }
    }

    let fifths = fifths.ok_or_else(|| ParseError::MissingElement {
        parent: "key".to_string(),
        element: "fifths".to_string(),
        position: reader.position(),
    })?;

    Ok(Key {
        number,
        content: KeyContent::Traditional(TraditionalKey { cancel, fifths, mode }),
        print_style: None,
        print_object,
        id,
    })
}

fn parse_time(reader: &mut XmlReader, start: &BytesStart) -> Result<Time, ParseError> {
    let number = reader.get_optional_attr_as(start, "number");
    let symbol = reader.get_optional_attr(start, "symbol")
        .and_then(|s| parse_time_symbol(&s).ok());
    let print_object = reader.get_optional_attr(start, "print-object")
        .and_then(|s| parse_yes_no(&s).ok());
    let id = reader.get_optional_attr(start, "id");

    let mut beats_list = Vec::new();
    let mut beat_type_list = Vec::new();
    let mut senza_misura = None;

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"beats" => beats_list.push(reader.read_text()?),
                    b"beat-type" => beat_type_list.push(reader.read_text()?),
                    b"senza-misura" => senza_misura = Some(reader.read_text().ok()),
                    _ => reader.skip_element()?,
                }
            }
            Event::End(e) if e.name().as_ref() == b"time" => break,
            _ => {}
        }
    }

    let content = if let Some(sm) = senza_misura {
        TimeContent::SenzaMisura(sm)
    } else {
        let signatures: Vec<TimeSignature> = beats_list
            .into_iter()
            .zip(beat_type_list.into_iter())
            .map(|(beats, beat_type)| TimeSignature { beats, beat_type })
            .collect();
        TimeContent::Measured { signatures, interchangeable: None }
    };

    Ok(Time {
        number,
        symbol,
        separator: None,
        content,
        print_style: None,
        print_object,
        id,
    })
}

fn parse_clef(reader: &mut XmlReader, start: &BytesStart) -> Result<Clef, ParseError> {
    let number = reader.get_optional_attr_as(start, "number");
    let print_object = reader.get_optional_attr(start, "print-object")
        .and_then(|s| parse_yes_no(&s).ok());
    let id = reader.get_optional_attr(start, "id");

    let mut sign = None;
    let mut line = None;
    let mut clef_octave_change = None;

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"sign" => sign = Some(parse_clef_sign(&reader.read_text()?)?),
                    b"line" => line = Some(reader.read_text_as()?),
                    b"clef-octave-change" => clef_octave_change = Some(reader.read_text_as()?),
                    _ => reader.skip_element()?,
                }
            }
            Event::End(e) if e.name().as_ref() == b"clef" => break,
            _ => {}
        }
    }

    let sign = sign.ok_or_else(|| ParseError::MissingElement {
        parent: "clef".to_string(),
        element: "sign".to_string(),
        position: reader.position(),
    })?;

    Ok(Clef {
        number,
        additional: None,
        size: None,
        after_barline: None,
        sign,
        line,
        clef_octave_change,
        print_style: None,
        print_object,
        id,
    })
}
```

**Acceptance Criteria:**

- [ ] Parse traditional key signatures (fifths + mode)
- [ ] Parse time signatures (beats + beat-type, senza-misura)
- [ ] Parse clefs (sign + line + octave-change)
- [ ] Parse divisions
- [ ] Handle multiple keys/times/clefs for different staves

---

## Task 2.2: Implement Core Note Parsing

The `<note>` element has three content variants and many optional children.

### Element Structure Examples

```xml
<!-- Regular note -->
<note>
  <pitch><step>C</step><octave>4</octave></pitch>
  <duration>1</duration>
  <voice>1</voice>
  <type>quarter</type>
</note>

<!-- Chord note (shares time with previous) -->
<note>
  <chord/>
  <pitch><step>E</step><octave>4</octave></pitch>
  <duration>1</duration>
  <type>quarter</type>
</note>

<!-- Grace note (no duration element) -->
<note>
  <grace slash="yes"/>
  <pitch><step>D</step><octave>4</octave></pitch>
  <type>eighth</type>
</note>

<!-- Rest -->
<note>
  <rest/>
  <duration>1</duration>
  <type>quarter</type>
</note>
```

### Implementation

```rust
fn parse_note(reader: &mut XmlReader, start: &BytesStart) -> Result<Note, ParseError> {
    // Collect all the pieces
    let mut grace = None;
    let mut cue = false;
    let mut chord = false;
    let mut pitch = None;
    let mut rest = None;
    let mut unpitched = None;
    let mut duration = None;
    let mut ties = Vec::new();
    let mut voice = None;
    let mut note_type = None;
    let mut dots = Vec::new();
    let mut accidental = None;
    let mut time_modification = None;
    let mut stem = None;
    let mut notehead = None;
    let mut staff = None;
    let mut beams = Vec::new();
    let mut notations = Vec::new();
    let mut lyrics = Vec::new();

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    // Note content variants
                    b"grace" => grace = Some(parse_grace(reader, &e)?),
                    b"cue" => { cue = true; reader.skip_element()?; }
                    b"chord" => { chord = true; reader.skip_element()?; }
                    b"pitch" => pitch = Some(parse_pitch(reader)?),
                    b"rest" => rest = Some(parse_rest(reader, &e)?),
                    b"unpitched" => unpitched = Some(parse_unpitched(reader)?),
                    b"duration" => duration = Some(reader.read_text_as()?),
                    b"tie" => ties.push(parse_tie(reader, &e)?),

                    // Common note elements
                    b"voice" => voice = Some(reader.read_text()?),
                    b"type" => note_type = Some(parse_note_type(reader, &e)?),
                    b"dot" => dots.push(parse_dot(reader, &e)?),
                    b"accidental" => accidental = Some(parse_accidental(reader, &e)?),
                    b"time-modification" => time_modification = Some(parse_time_modification(reader)?),
                    b"stem" => stem = Some(parse_stem(reader, &e)?),
                    b"notehead" => notehead = Some(parse_notehead(reader, &e)?),
                    b"staff" => staff = Some(reader.read_text_as()?),
                    b"beam" => beams.push(parse_beam(reader, &e)?),
                    b"notations" => notations.push(parse_notations(reader)?),
                    b"lyric" => lyrics.push(parse_lyric(reader, &e)?),

                    _ => reader.skip_element()?,
                }
            }
            Event::Empty(e) => {
                // Handle self-closing elements like <chord/>, <rest/>, <dot/>
                match e.name().as_ref() {
                    b"grace" => grace = Some(parse_grace_from_empty(&e)?),
                    b"cue" => cue = true,
                    b"chord" => chord = true,
                    b"rest" => rest = Some(parse_rest_from_empty(&e)?),
                    b"dot" => dots.push(parse_dot_from_empty(&e)?),
                    b"tie" => ties.push(parse_tie_from_empty(&e)?),
                    _ => {}
                }
            }
            Event::End(e) if e.name().as_ref() == b"note" => break,
            _ => {}
        }
    }

    // Build full note content
    let full_note = FullNote {
        chord,
        content: if let Some(p) = pitch {
            PitchRestUnpitched::Pitch(p)
        } else if let Some(r) = rest {
            PitchRestUnpitched::Rest(r)
        } else if let Some(u) = unpitched {
            PitchRestUnpitched::Unpitched(u)
        } else {
            return Err(ParseError::MissingElement {
                parent: "note".to_string(),
                element: "pitch, rest, or unpitched".to_string(),
                position: reader.position(),
            });
        },
    };

    // Determine note content variant
    let content = if let Some(g) = grace {
        NoteContent::Grace { grace: g, full_note, ties }
    } else if cue {
        let dur = duration.ok_or_else(|| ParseError::MissingElement {
            parent: "cue note".to_string(),
            element: "duration".to_string(),
            position: reader.position(),
        })?;
        NoteContent::Cue { full_note, duration: dur }
    } else {
        let dur = duration.ok_or_else(|| ParseError::MissingElement {
            parent: "note".to_string(),
            element: "duration".to_string(),
            position: reader.position(),
        })?;
        NoteContent::Regular { full_note, duration: dur, ties }
    };

    Ok(Note {
        content,
        instrument: None,
        editorial_voice: None,
        voice,
        r#type: note_type,
        dots,
        accidental,
        time_modification,
        stem,
        notehead,
        notehead_text: None,
        staff,
        beams,
        notations,
        lyrics,
        play: None,
        listen: None,
    })
}

fn parse_pitch(reader: &mut XmlReader) -> Result<Pitch, ParseError> {
    let mut step = None;
    let mut alter = None;
    let mut octave = None;

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"step" => step = Some(parse_step(&reader.read_text()?)?),
                    b"alter" => alter = Some(reader.read_text_as()?),
                    b"octave" => octave = Some(reader.read_text_as()?),
                    _ => reader.skip_element()?,
                }
            }
            Event::End(e) if e.name().as_ref() == b"pitch" => break,
            _ => {}
        }
    }

    Ok(Pitch {
        step: step.ok_or_else(|| ParseError::MissingElement {
            parent: "pitch".to_string(),
            element: "step".to_string(),
            position: reader.position(),
        })?,
        alter,
        octave: octave.ok_or_else(|| ParseError::MissingElement {
            parent: "pitch".to_string(),
            element: "octave".to_string(),
            position: reader.position(),
        })?,
    })
}

fn parse_rest(reader: &mut XmlReader, start: &BytesStart) -> Result<Rest, ParseError> {
    let measure = reader.get_optional_attr(start, "measure")
        .and_then(|s| parse_yes_no(&s).ok())
        .unwrap_or(false);

    let mut display_step = None;
    let mut display_octave = None;

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"display-step" => display_step = Some(parse_step(&reader.read_text()?)?),
                    b"display-octave" => display_octave = Some(reader.read_text_as()?),
                    _ => reader.skip_element()?,
                }
            }
            Event::End(e) if e.name().as_ref() == b"rest" => break,
            _ => {}
        }
    }

    Ok(Rest { display_step, display_octave, measure })
}

fn parse_rest_from_empty(start: &BytesStart) -> Result<Rest, ParseError> {
    let measure = get_optional_attr(start, "measure")
        .and_then(|s| parse_yes_no(&s).ok())
        .unwrap_or(false);
    Ok(Rest { display_step: None, display_octave: None, measure })
}

fn parse_note_type(reader: &mut XmlReader, start: &BytesStart) -> Result<NoteType, ParseError> {
    let size = reader.get_optional_attr(start, "size"); // TODO: parse SymbolSize
    let value = parse_note_type_value(&reader.read_text()?)?;
    Ok(NoteType { value, size: None })
}
```

**Acceptance Criteria:**

- [ ] Parse regular notes with pitch and duration
- [ ] Parse chord notes (with `<chord/>` flag)
- [ ] Parse grace notes (no duration)
- [ ] Parse cue notes
- [ ] Parse rests (with optional display position)
- [ ] Handle both `<rest>...</rest>` and `<rest/>` forms

---

## Task 2.3: Implement Tie, Grace, Accidental, Time Modification

### Implementation

```rust
fn parse_tie(reader: &mut XmlReader, start: &BytesStart) -> Result<Tie, ParseError> {
    let type_str = reader.get_attr(start, "type")?;
    let r#type = parse_start_stop(&type_str)?;
    let time_only = reader.get_optional_attr(start, "time-only");
    reader.skip_element()?;
    Ok(Tie { r#type, time_only })
}

fn parse_tie_from_empty(start: &BytesStart) -> Result<Tie, ParseError> {
    let type_str = get_attr(start, "type", 0)?;
    Ok(Tie {
        r#type: parse_start_stop(&type_str)?,
        time_only: get_optional_attr(start, "time-only"),
    })
}

fn parse_grace(reader: &mut XmlReader, start: &BytesStart) -> Result<Grace, ParseError> {
    let slash = reader.get_optional_attr(start, "slash")
        .and_then(|s| parse_yes_no(&s).ok());
    let steal_time_previous = reader.get_optional_attr_as(start, "steal-time-previous");
    let steal_time_following = reader.get_optional_attr_as(start, "steal-time-following");
    let make_time = reader.get_optional_attr_as(start, "make-time");
    reader.skip_element()?;
    Ok(Grace { slash, steal_time_previous, steal_time_following, make_time })
}

fn parse_grace_from_empty(start: &BytesStart) -> Result<Grace, ParseError> {
    Ok(Grace {
        slash: get_optional_attr(start, "slash").and_then(|s| parse_yes_no(&s).ok()),
        steal_time_previous: get_optional_attr(start, "steal-time-previous")
            .and_then(|s| s.parse().ok()),
        steal_time_following: get_optional_attr(start, "steal-time-following")
            .and_then(|s| s.parse().ok()),
        make_time: get_optional_attr(start, "make-time").and_then(|s| s.parse().ok()),
    })
}

fn parse_accidental(reader: &mut XmlReader, start: &BytesStart) -> Result<Accidental, ParseError> {
    let cautionary = reader.get_optional_attr(start, "cautionary")
        .and_then(|s| parse_yes_no(&s).ok());
    let editorial = reader.get_optional_attr(start, "editorial")
        .and_then(|s| parse_yes_no(&s).ok());
    let parentheses = reader.get_optional_attr(start, "parentheses")
        .and_then(|s| parse_yes_no(&s).ok());
    let bracket = reader.get_optional_attr(start, "bracket")
        .and_then(|s| parse_yes_no(&s).ok());

    let value = parse_accidental_value(&reader.read_text()?)?;

    Ok(Accidental {
        value,
        cautionary,
        editorial,
        parentheses,
        bracket,
        size: None,
        print_style: None,
        smufl: None,
    })
}

fn parse_time_modification(reader: &mut XmlReader) -> Result<TimeModification, ParseError> {
    let mut actual_notes = None;
    let mut normal_notes = None;
    let mut normal_type = None;
    let mut normal_dots = 0;

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"actual-notes" => actual_notes = Some(reader.read_text_as()?),
                    b"normal-notes" => normal_notes = Some(reader.read_text_as()?),
                    b"normal-type" => {
                        normal_type = Some(parse_note_type_value(&reader.read_text()?)?);
                    }
                    b"normal-dot" => { normal_dots += 1; reader.skip_element()?; }
                    _ => reader.skip_element()?,
                }
            }
            Event::Empty(e) if e.name().as_ref() == b"normal-dot" => {
                normal_dots += 1;
            }
            Event::End(e) if e.name().as_ref() == b"time-modification" => break,
            _ => {}
        }
    }

    Ok(TimeModification {
        actual_notes: actual_notes.ok_or_else(|| ParseError::MissingElement {
            parent: "time-modification".to_string(),
            element: "actual-notes".to_string(),
            position: 0,
        })?,
        normal_notes: normal_notes.ok_or_else(|| ParseError::MissingElement {
            parent: "time-modification".to_string(),
            element: "normal-notes".to_string(),
            position: 0,
        })?,
        normal_type,
        normal_dots,
    })
}
```

**Acceptance Criteria:**

- [ ] Parse tie start/stop
- [ ] Parse grace note attributes (slash, steal-time)
- [ ] Parse accidentals with all attributes
- [ ] Parse time modification (tuplet ratios)

---

## Task 2.4: Implement Beam, Stem, Notehead, Dot

### Implementation

```rust
fn parse_beam(reader: &mut XmlReader, start: &BytesStart) -> Result<Beam, ParseError> {
    let number = reader.get_optional_attr_as(start, "number").unwrap_or(1);
    let repeater = reader.get_optional_attr(start, "repeater")
        .and_then(|s| parse_yes_no(&s).ok());
    let fan = reader.get_optional_attr(start, "fan"); // TODO: parse Fan enum

    let value = parse_beam_value(&reader.read_text()?)?;

    Ok(Beam {
        value,
        number,
        repeater,
        fan: None,
        color: None,
        id: None,
    })
}

fn parse_stem(reader: &mut XmlReader, start: &BytesStart) -> Result<Stem, ParseError> {
    // TODO: parse position and color attributes
    let value = parse_stem_value(&reader.read_text()?)?;
    Ok(Stem { value, position: None, color: None })
}

fn parse_notehead(reader: &mut XmlReader, start: &BytesStart) -> Result<Notehead, ParseError> {
    let filled = reader.get_optional_attr(start, "filled")
        .and_then(|s| parse_yes_no(&s).ok());
    let parentheses = reader.get_optional_attr(start, "parentheses")
        .and_then(|s| parse_yes_no(&s).ok());

    let value = parse_notehead_value(&reader.read_text()?)?;

    Ok(Notehead {
        value,
        filled,
        parentheses,
        font: None,
        color: None,
        smufl: None,
    })
}

fn parse_dot(reader: &mut XmlReader, start: &BytesStart) -> Result<Dot, ParseError> {
    // TODO: parse print-style, placement attributes
    reader.skip_element()?;
    Ok(Dot {
        print_style: None,
        placement: None,
    })
}

fn parse_dot_from_empty(start: &BytesStart) -> Result<Dot, ParseError> {
    Ok(Dot {
        print_style: None,
        placement: None,
    })
}
```

**Acceptance Criteria:**

- [ ] Parse beam values (begin, continue, end, forward/backward hook)
- [ ] Parse stem directions (up, down, none, double)
- [ ] Parse notehead shapes
- [ ] Parse dot elements

---

## Task 2.5: Write Round-Trip Test

This is the critical validation that parser and emitter work together correctly.

### Test Implementation

```rust
#[cfg(test)]
mod round_trip_tests {
    use crate::ir::*;
    use crate::musicxml::{emit, parse};

    /// The fundamental round-trip test: emit → parse → emit should produce identical XML
    #[test]
    fn test_round_trip_simple_score() {
        let score = build_twinkle_score();

        // First emission
        let xml1 = emit(&score).expect("first emit should succeed");

        // Parse back to IR
        let parsed = parse(&xml1).expect("parse should succeed");

        // Second emission
        let xml2 = emit(&parsed).expect("second emit should succeed");

        // XML should be identical
        assert_eq!(xml1, xml2, "Round-trip XML should be identical");
    }

    /// Test that parsed IR equals original IR
    #[test]
    fn test_round_trip_ir_equality() {
        let score = build_twinkle_score();
        let xml = emit(&score).expect("emit should succeed");
        let parsed = parse(&xml).expect("parse should succeed");

        // Compare IR structures
        assert_eq!(score.version, parsed.version);
        assert_eq!(score.parts.len(), parsed.parts.len());
        assert_eq!(
            score.parts[0].measures.len(),
            parsed.parts[0].measures.len()
        );
    }

    /// Test parsing real MusicXML and re-emitting
    #[test]
    fn test_round_trip_from_xml() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE score-partwise PUBLIC "-//Recordare//DTD MusicXML 4.0 Partwise//EN" "http://www.musicxml.org/dtds/partwise.dtd">
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <attributes>
        <divisions>1</divisions>
        <key><fifths>0</fifths><mode>major</mode></key>
        <time><beats>4</beats><beat-type>4</beat-type></time>
        <clef><sign>G</sign><line>2</line></clef>
      </attributes>
      <note>
        <pitch><step>C</step><octave>4</octave></pitch>
        <duration>1</duration>
        <voice>1</voice>
        <type>quarter</type>
      </note>
      <note>
        <pitch><step>D</step><octave>4</octave></pitch>
        <duration>1</duration>
        <voice>1</voice>
        <type>quarter</type>
      </note>
      <note>
        <pitch><step>E</step><octave>4</octave></pitch>
        <duration>1</duration>
        <voice>1</voice>
        <type>quarter</type>
      </note>
      <note>
        <pitch><step>F</step><octave>4</octave></pitch>
        <duration>1</duration>
        <voice>1</voice>
        <type>quarter</type>
      </note>
    </measure>
  </part>
</score-partwise>"#;

        // Parse
        let parsed = parse(xml).expect("parse should succeed");

        // Verify structure
        assert_eq!(parsed.parts.len(), 1);
        assert_eq!(parsed.parts[0].measures.len(), 1);

        let measure = &parsed.parts[0].measures[0];
        let note_count = measure.content.iter()
            .filter(|e| matches!(e, MusicDataElement::Note(_)))
            .count();
        assert_eq!(note_count, 4);

        // Re-emit and re-parse (double round-trip)
        let re_emitted = emit(&parsed).expect("re-emit should succeed");
        let re_parsed = parse(&re_emitted).expect("re-parse should succeed");

        // Should be stable after two round-trips
        let final_xml = emit(&re_parsed).expect("final emit should succeed");
        assert_eq!(re_emitted, final_xml, "Should be stable after double round-trip");
    }

    /// Test specific note features round-trip correctly
    #[test]
    fn test_round_trip_note_features() {
        // Test accidentals
        let xml_sharp = make_note_xml("C", "4", Some("sharp"), "quarter");
        let parsed = parse(&xml_sharp).unwrap();
        let note = get_first_note(&parsed);
        assert!(note.accidental.is_some());

        // Test dots
        let xml_dotted = make_dotted_note_xml("D", "4", 1);
        let parsed = parse(&xml_dotted).unwrap();
        let note = get_first_note(&parsed);
        assert_eq!(note.dots.len(), 1);
    }

    /// Test chord round-trip
    #[test]
    fn test_round_trip_chord() {
        let xml = r#"<?xml version="1.0"?>
<score-partwise version="4.0">
  <part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list>
  <part id="P1">
    <measure number="1">
      <attributes><divisions>1</divisions></attributes>
      <note>
        <pitch><step>C</step><octave>4</octave></pitch>
        <duration>1</duration><type>quarter</type>
      </note>
      <note>
        <chord/>
        <pitch><step>E</step><octave>4</octave></pitch>
        <duration>1</duration><type>quarter</type>
      </note>
      <note>
        <chord/>
        <pitch><step>G</step><octave>4</octave></pitch>
        <duration>1</duration><type>quarter</type>
      </note>
    </measure>
  </part>
</score-partwise>"#;

        let parsed = parse(xml).expect("should parse chord");

        // Verify chord flags
        let notes: Vec<_> = parsed.parts[0].measures[0].content.iter()
            .filter_map(|e| match e {
                MusicDataElement::Note(n) => Some(n),
                _ => None,
            })
            .collect();

        assert_eq!(notes.len(), 3);
        assert!(!notes[0].content.is_chord()); // First note not a chord
        assert!(notes[1].content.is_chord());  // Second note is chord
        assert!(notes[2].content.is_chord());  // Third note is chord

        // Round-trip
        let re_emitted = emit(&parsed).unwrap();
        let re_parsed = parse(&re_emitted).unwrap();
        let final_xml = emit(&re_parsed).unwrap();
        assert_eq!(re_emitted, final_xml);
    }

    // Helper functions
    fn build_twinkle_score() -> ScorePartwise {
        // Same as Phase 2 test helper - builds "Twinkle Twinkle" first phrase
        // ... (implementation from Phase 2)
        todo!("Copy from Phase 2 test helpers")
    }

    fn get_first_note(score: &ScorePartwise) -> &Note {
        score.parts[0].measures[0].content.iter()
            .find_map(|e| match e {
                MusicDataElement::Note(n) => Some(n),
                _ => None,
            })
            .expect("should have a note")
    }

    fn make_note_xml(step: &str, octave: &str, accidental: Option<&str>, note_type: &str) -> String {
        let acc_xml = accidental
            .map(|a| format!("<accidental>{}</accidental>", a))
            .unwrap_or_default();
        format!(r#"<?xml version="1.0"?>
<score-partwise version="4.0">
  <part-list><score-part id="P1"><part-name>Test</part-name></score-part></part-list>
  <part id="P1">
    <measure number="1">
      <attributes><divisions>1</divisions></attributes>
      <note>
        <pitch><step>{}</step><octave>{}</octave></pitch>
        <duration>1</duration>
        <type>{}</type>
        {}
      </note>
    </measure>
  </part>
</score-partwise>"#, step, octave, note_type, acc_xml)
    }

    fn make_dotted_note_xml(step: &str, octave: &str, num_dots: usize) -> String {
        let dots_xml = "<dot/>".repeat(num_dots);
        format!(r#"<?xml version="1.0"?>
<score-partwise version="4.0">
  <part-list><score-part id="P1"><part-name>Test</part-name></score-part></part-list>
  <part id="P1">
    <measure number="1">
      <attributes><divisions>1</divisions></attributes>
      <note>
        <pitch><step>{}</step><octave>{}</octave></pitch>
        <duration>1</duration>
        <type>quarter</type>
        {}
      </note>
    </measure>
  </part>
</score-partwise>"#, step, octave, dots_xml)
    }
}
```

**Acceptance Criteria:**

- [ ] `emit → parse → emit` produces identical XML
- [ ] Parsed IR matches original IR structure
- [ ] Double round-trip is stable
- [ ] Chord notes preserve `<chord/>` flag
- [ ] Accidentals round-trip correctly
- [ ] Dotted notes round-trip correctly

---

## Milestone 2 Summary

After completing Milestone 2:

1. **Attributes parsing** — key, time, clef, divisions
2. **Note parsing** — all three variants (regular, grace, cue)
3. **Supporting elements** — pitch, rest, tie, accidental, beam, stem, etc.
4. **Round-trip validation** — parser and emitter work together

You can now parse and round-trip simple MusicXML scores.

---

*Next document: Milestone 3 — Voice, Barlines, Navigation*

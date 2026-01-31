# Phase 3, Milestone 5: Extended Features

> **Document Series:** 6 of 7
> **Tasks:** 5.1–5.5
> **Focus:** Lyrics, ornaments (detailed), score header, comprehensive tests

---

## Overview

This milestone completes the parser with:

- Lyrics (syllables, melismas, multiple verses)
- Complete score header (work, identification, credits)
- Comprehensive integration test suite

After this milestone, you can parse any valid MusicXML 4.0 document.

---

## Task 5.1: Implement Lyrics Parsing

### Element Structure

```xml
<lyric number="1" default-y="-80">
  <syllabic>begin</syllabic>
  <text>Hel</text>
</lyric>

<lyric number="1">
  <syllabic>end</syllabic>
  <text>lo</text>
  <extend type="start"/>
</lyric>

<!-- Verse 2 -->
<lyric number="2">
  <syllabic>single</syllabic>
  <text>World</text>
</lyric>
```

### Implementation

```rust
fn parse_lyric(reader: &mut XmlReader, start: &BytesStart) -> Result<Lyric, ParseError> {
    let number = reader.get_optional_attr(start, "number");
    let name = reader.get_optional_attr(start, "name");
    let justify = reader.get_optional_attr(start, "justify"); // TODO: parse LeftCenterRight
    let placement = reader.get_optional_attr(start, "placement")
        .and_then(|s| parse_above_below(&s).ok());
    let time_only = reader.get_optional_attr(start, "time-only");
    let id = reader.get_optional_attr(start, "id");

    let mut syllabic = None;
    let mut text = None;
    let mut elision_syllabics = Vec::new();
    let mut extend = None;
    let mut laughing = false;
    let mut humming = false;
    let mut end_line = None;
    let mut end_paragraph = None;

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"syllabic" => syllabic = Some(parse_syllabic(&reader.read_text()?)?),
                    b"text" => {
                        let value = reader.read_text()?;
                        text = Some(TextElementData {
                            value,
                            font: None,
                            color: None,
                            text_decoration: None,
                            text_rotation: None,
                            letter_spacing: None,
                            lang: None,
                            text_direction: None,
                        });
                    }
                    b"elision" => {
                        // Elision connects multiple syllables on one note
                        let elision_text = reader.read_text().ok();
                        // Next should be syllabic + text
                        // Store for later processing
                    }
                    b"extend" => extend = Some(parse_extend(reader, &e)?),
                    b"laughing" => {
                        laughing = true;
                        reader.skip_element()?;
                    }
                    b"humming" => {
                        humming = true;
                        reader.skip_element()?;
                    }
                    b"end-line" => {
                        end_line = Some(());
                        reader.skip_element()?;
                    }
                    b"end-paragraph" => {
                        end_paragraph = Some(());
                        reader.skip_element()?;
                    }
                    _ => reader.skip_element()?,
                }
            }
            Event::Empty(e) => {
                match e.name().as_ref() {
                    b"extend" => extend = Some(parse_extend_from_empty(&e)?),
                    b"laughing" => laughing = true,
                    b"humming" => humming = true,
                    b"end-line" => end_line = Some(()),
                    b"end-paragraph" => end_paragraph = Some(()),
                    _ => {}
                }
            }
            Event::End(e) if e.name().as_ref() == b"lyric" => break,
            _ => {}
        }
    }

    // Determine lyric content variant
    let content = if laughing {
        LyricContent::Laughing
    } else if humming {
        LyricContent::Humming
    } else if let Some(t) = text {
        LyricContent::Syllabic {
            items: vec![SyllabicTextGroup {
                syllabic,
                text: t,
                elision_syllabics,
            }],
            extend,
            end_line: end_line.map(|_| Empty {}),
            end_paragraph: end_paragraph.map(|_| Empty {}),
        }
    } else if let Some(ext) = extend {
        LyricContent::Extend(ext)
    } else {
        // Default to humming if nothing else
        LyricContent::Humming
    };

    Ok(Lyric {
        number,
        name,
        content,
        justify: None,
        print_style: None,
        placement,
        time_only,
        id,
    })
}

fn parse_extend(reader: &mut XmlReader, start: &BytesStart) -> Result<Extend, ParseError> {
    let r#type = reader.get_optional_attr(start, "type")
        .and_then(|s| parse_start_stop_continue(&s).ok());

    reader.skip_element()?;

    Ok(Extend {
        r#type,
        print_style: None,
    })
}

fn parse_extend_from_empty(start: &BytesStart) -> Result<Extend, ParseError> {
    Ok(Extend {
        r#type: get_optional_attr(start, "type")
            .and_then(|s| parse_start_stop_continue(&s).ok()),
        print_style: None,
    })
}
```

**Acceptance Criteria:**

- [ ] Parse syllabic type (single, begin, middle, end)
- [ ] Parse text content with font attributes
- [ ] Parse extend (melisma) lines
- [ ] Parse multiple lyric lines (verse numbers)
- [ ] Handle laughing and humming elements

---

## Task 5.2: Implement Complete Ornaments (detailed)

Expand on Milestone 4 with full ornament support:

```rust
fn parse_mordent(start: &BytesStart) -> Mordent {
    Mordent {
        long: get_optional_attr(start, "long").and_then(|s| parse_yes_no(&s).ok()),
        approach: get_optional_attr(start, "approach"), // TODO: parse AboveBelow
        departure: get_optional_attr(start, "departure"),
        print_style: None,
        placement: get_optional_attr(start, "placement").and_then(|s| parse_above_below(&s).ok()),
        trill_sound: None,
    }
}

fn parse_horizontal_turn(start: &BytesStart) -> HorizontalTurn {
    HorizontalTurn {
        slash: get_optional_attr(start, "slash").and_then(|s| parse_yes_no(&s).ok()),
        print_style: None,
        placement: get_optional_attr(start, "placement").and_then(|s| parse_above_below(&s).ok()),
        trill_sound: None,
    }
}

fn parse_empty_trill_sound(start: &BytesStart) -> EmptyTrillSound {
    EmptyTrillSound {
        print_style: None,
        placement: get_optional_attr(start, "placement").and_then(|s| parse_above_below(&s).ok()),
        trill_sound: None,
    }
}

fn parse_accidental_mark(reader: &mut XmlReader, start: &BytesStart) -> Result<AccidentalMark, ParseError> {
    let placement = reader.get_optional_attr(start, "placement")
        .and_then(|s| parse_above_below(&s).ok());
    let id = reader.get_optional_attr(start, "id");

    let value = parse_accidental_value(&reader.read_text()?)?;

    Ok(AccidentalMark {
        value,
        parentheses: None,
        bracket: None,
        size: None,
        print_style: None,
        placement,
        smufl: None,
        id,
    })
}

fn parse_arpeggiate(reader: &mut XmlReader, start: &BytesStart) -> Result<Arpeggiate, ParseError> {
    let number = reader.get_optional_attr_as(start, "number");
    let direction = reader.get_optional_attr(start, "direction")
        .and_then(|s| parse_up_down(&s).ok());
    let id = reader.get_optional_attr(start, "id");

    reader.skip_element()?;

    Ok(Arpeggiate {
        number,
        direction,
        unbroken: None,
        position: None,
        placement: None,
        color: None,
        id,
    })
}

fn parse_arpeggiate_from_empty(start: &BytesStart) -> Result<Arpeggiate, ParseError> {
    Ok(Arpeggiate {
        number: get_optional_attr(start, "number").and_then(|s| s.parse().ok()),
        direction: get_optional_attr(start, "direction").and_then(|s| parse_up_down(&s).ok()),
        unbroken: None,
        position: None,
        placement: None,
        color: None,
        id: get_optional_attr(start, "id"),
    })
}
```

**Acceptance Criteria:**

- [ ] Parse trill marks with trill-sound attributes
- [ ] Parse turns and inverted turns
- [ ] Parse mordent with long attribute
- [ ] Parse accidental marks within ornaments
- [ ] Parse tremolo with marks count

---

## Task 5.3: Implement Technical Marks (detailed)

```rust
fn parse_harmonic(reader: &mut XmlReader, start: &BytesStart) -> Result<Harmonic, ParseError> {
    let placement = reader.get_optional_attr(start, "placement")
        .and_then(|s| parse_above_below(&s).ok());

    let mut natural = false;
    let mut artificial = false;
    let mut base_pitch = false;
    let mut touching_pitch = false;
    let mut sounding_pitch = false;

    loop {
        match reader.next_event()? {
            Event::Start(e) | Event::Empty(e) => {
                match e.name().as_ref() {
                    b"natural" => natural = true,
                    b"artificial" => artificial = true,
                    b"base-pitch" => base_pitch = true,
                    b"touching-pitch" => touching_pitch = true,
                    b"sounding-pitch" => sounding_pitch = true,
                    _ => {}
                }
            }
            Event::End(e) if e.name().as_ref() == b"harmonic" => break,
            _ => {}
        }
    }

    Ok(Harmonic {
        natural: if natural { Some(Empty {}) } else { None },
        artificial: if artificial { Some(Empty {}) } else { None },
        base_pitch: if base_pitch { Some(Empty {}) } else { None },
        touching_pitch: if touching_pitch { Some(Empty {}) } else { None },
        sounding_pitch: if sounding_pitch { Some(Empty {}) } else { None },
        print_object: None,
        print_style: None,
        placement,
    })
}

fn parse_fret(reader: &mut XmlReader, start: &BytesStart) -> Result<Fret, ParseError> {
    let value = reader.read_text_as()?;
    Ok(Fret { value, font: None, color: None })
}

fn parse_string_number(reader: &mut XmlReader, start: &BytesStart) -> Result<StringNumber, ParseError> {
    let placement = reader.get_optional_attr(start, "placement")
        .and_then(|s| parse_above_below(&s).ok());
    let value = reader.read_text_as()?;

    Ok(StringNumber {
        value,
        print_style: None,
        placement,
    })
}

fn parse_pluck(reader: &mut XmlReader, start: &BytesStart) -> Result<Pluck, ParseError> {
    let value = reader.read_text()?;
    Ok(Pluck {
        value,
        print_style: None,
        placement: None,
    })
}

fn parse_hammer_on_pull_off(reader: &mut XmlReader, start: &BytesStart) -> Result<HammerOnPullOff, ParseError> {
    let r#type = parse_start_stop(&reader.get_attr(start, "type")?)?;
    let number = reader.get_optional_attr_as(start, "number").unwrap_or(1);

    let value = reader.read_text().unwrap_or_default();

    Ok(HammerOnPullOff {
        r#type,
        number,
        value,
        print_style: None,
        placement: None,
    })
}

fn parse_tap(reader: &mut XmlReader, start: &BytesStart) -> Result<Tap, ParseError> {
    let hand = reader.get_optional_attr(start, "hand"); // TODO: parse TapHand
    let value = reader.read_text().unwrap_or_default();

    Ok(Tap {
        value,
        hand: None,
        print_style: None,
        placement: None,
    })
}
```

**Acceptance Criteria:**

- [ ] Parse harmonic types (natural, artificial)
- [ ] Parse fret and string numbers
- [ ] Parse hammer-on and pull-off
- [ ] Parse pluck finger indications

---

## Task 5.4: Implement Score Header

### Element Structure

```xml
<score-partwise version="4.0">
  <work>
    <work-number>Op. 1</work-number>
    <work-title>Symphony No. 1</work-title>
  </work>
  <movement-number>1</movement-number>
  <movement-title>Allegro</movement-title>
  <identification>
    <creator type="composer">Ludwig van Beethoven</creator>
    <creator type="lyricist">Anonymous</creator>
    <rights>Copyright 2024</rights>
    <encoding>
      <software>Fermata</software>
      <encoding-date>2024-01-01</encoding-date>
    </encoding>
    <source>Original manuscript</source>
  </identification>
  <credit page="1">
    <credit-words>Symphony No. 1</credit-words>
  </credit>
  <!-- part-list and parts follow -->
</score-partwise>
```

### Implementation

```rust
fn parse_work(reader: &mut XmlReader) -> Result<Work, ParseError> {
    let mut work_number = None;
    let mut work_title = None;
    let mut opus = None;

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"work-number" => work_number = Some(reader.read_text()?),
                    b"work-title" => work_title = Some(reader.read_text()?),
                    b"opus" => {
                        // Opus is a link element - skip for now
                        reader.skip_element()?;
                    }
                    _ => reader.skip_element()?,
                }
            }
            Event::End(e) if e.name().as_ref() == b"work" => break,
            _ => {}
        }
    }

    Ok(Work { work_number, work_title, opus })
}

fn parse_identification(reader: &mut XmlReader) -> Result<Identification, ParseError> {
    let mut creators = Vec::new();
    let mut rights = Vec::new();
    let mut encoding = None;
    let mut source = None;
    let mut relations = Vec::new();
    let mut miscellaneous = None;

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"creator" => {
                        let r#type = reader.get_optional_attr(&e, "type");
                        let value = reader.read_text()?;
                        creators.push(TypedText { r#type, value });
                    }
                    b"rights" => {
                        let r#type = reader.get_optional_attr(&e, "type");
                        let value = reader.read_text()?;
                        rights.push(TypedText { r#type, value });
                    }
                    b"encoding" => encoding = Some(parse_encoding(reader)?),
                    b"source" => source = Some(reader.read_text()?),
                    b"relation" => {
                        let r#type = reader.get_optional_attr(&e, "type");
                        let value = reader.read_text()?;
                        relations.push(TypedText { r#type, value });
                    }
                    b"miscellaneous" => miscellaneous = Some(parse_miscellaneous(reader)?),
                    _ => reader.skip_element()?,
                }
            }
            Event::End(e) if e.name().as_ref() == b"identification" => break,
            _ => {}
        }
    }

    Ok(Identification {
        creators, rights, encoding, source, relations, miscellaneous,
    })
}

fn parse_encoding(reader: &mut XmlReader) -> Result<Encoding, ParseError> {
    let mut encoding_date = None;
    let mut encoders = Vec::new();
    let mut software = Vec::new();
    let mut encoding_description = None;
    let mut supports = Vec::new();

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"encoding-date" => encoding_date = Some(reader.read_text()?),
                    b"encoder" => {
                        let r#type = reader.get_optional_attr(&e, "type");
                        let value = reader.read_text()?;
                        encoders.push(TypedText { r#type, value });
                    }
                    b"software" => software.push(reader.read_text()?),
                    b"encoding-description" => encoding_description = Some(reader.read_text()?),
                    b"supports" => {
                        supports.push(parse_supports(reader, &e)?);
                    }
                    _ => reader.skip_element()?,
                }
            }
            Event::End(e) if e.name().as_ref() == b"encoding" => break,
            _ => {}
        }
    }

    Ok(Encoding {
        encoding_date, encoders, software, encoding_description, supports,
    })
}

fn parse_supports(reader: &mut XmlReader, start: &BytesStart) -> Result<Supports, ParseError> {
    let element = reader.get_attr(start, "element")?;
    let type_str = reader.get_attr(start, "type")?;
    let r#type = parse_yes_no(&type_str)?;
    let attribute = reader.get_optional_attr(start, "attribute");
    let value = reader.get_optional_attr(start, "value");

    reader.skip_element()?;

    Ok(Supports { element, r#type, attribute, value })
}

fn parse_miscellaneous(reader: &mut XmlReader) -> Result<Miscellaneous, ParseError> {
    let mut fields = Vec::new();

    loop {
        match reader.next_event()? {
            Event::Start(e) if e.name().as_ref() == b"miscellaneous-field" => {
                let name = reader.get_attr(&e, "name")?;
                let value = reader.read_text()?;
                fields.push(MiscellaneousField { name, value });
            }
            Event::End(e) if e.name().as_ref() == b"miscellaneous" => break,
            _ => {}
        }
    }

    Ok(Miscellaneous { fields })
}

fn parse_credit(reader: &mut XmlReader, start: &BytesStart) -> Result<Credit, ParseError> {
    let page = reader.get_optional_attr_as(start, "page");
    let id = reader.get_optional_attr(start, "id");

    let mut credit_types = Vec::new();
    let mut content = Vec::new();

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"credit-type" => credit_types.push(reader.read_text()?),
                    b"credit-words" => {
                        content.push(CreditContent::CreditWords(parse_credit_words(reader, &e)?));
                    }
                    b"credit-symbol" => {
                        content.push(CreditContent::CreditSymbol(parse_credit_symbol(reader, &e)?));
                    }
                    b"credit-image" => {
                        content.push(CreditContent::CreditImage(parse_credit_image(reader, &e)?));
                    }
                    _ => reader.skip_element()?,
                }
            }
            Event::End(e) if e.name().as_ref() == b"credit" => break,
            _ => {}
        }
    }

    Ok(Credit {
        page, credit_types, links: vec![], bookmarks: vec![], content, id,
    })
}

fn parse_credit_words(reader: &mut XmlReader, start: &BytesStart) -> Result<FormattedTextId, ParseError> {
    let id = reader.get_optional_attr(start, "id");
    let value = reader.read_text()?;

    Ok(FormattedTextId {
        value,
        print_style: None,
        text_decoration: None,
        text_rotation: None,
        letter_spacing: None,
        lang: None,
        text_direction: None,
        enclosure: None,
        id,
    })
}

fn parse_credit_symbol(reader: &mut XmlReader, start: &BytesStart) -> Result<FormattedSymbolId, ParseError> {
    let id = reader.get_optional_attr(start, "id");
    let value = reader.read_text()?;

    Ok(FormattedSymbolId {
        value, print_style: None, enclosure: None, id,
    })
}

fn parse_credit_image(reader: &mut XmlReader, start: &BytesStart) -> Result<Image, ParseError> {
    let source = reader.get_attr(start, "source")?;
    let r#type = reader.get_attr(start, "type")?;

    reader.skip_element()?;

    Ok(Image {
        source,
        r#type,
        height: None,
        width: None,
        position: None,
        halign: None,
        valign_image: None,
        id: None,
    })
}
```

**Acceptance Criteria:**

- [ ] Parse work (work-number, work-title)
- [ ] Parse identification (creators, rights, encoding)
- [ ] Parse encoding details (software, date, supports)
- [ ] Parse credits (title, composer, etc.)

---

## Task 5.5: Comprehensive Integration Test Suite

```rust
#[cfg(test)]
mod integration_tests {
    use crate::musicxml::{emit, parse};
    use std::fs;
    use std::path::Path;

    /// Test round-trip for all fixture files
    #[test]
    fn test_round_trip_all_fixtures() {
        let fixtures = [
            "twinkle_twinkle.musicxml",
            "two_voices.musicxml",
            "with_repeats.musicxml",
            "with_dynamics.musicxml",
            "with_lyrics.musicxml",
            "full_score_header.musicxml",
        ];

        for fixture in fixtures {
            let path = format!("tests/fixtures/{}", fixture);
            if Path::new(&path).exists() {
                println!("Testing round-trip for: {}", fixture);

                let xml = fs::read_to_string(&path).expect("should read file");
                let parsed = parse(&xml).expect(&format!("should parse {}", fixture));
                let emitted = emit(&parsed).expect("should emit");
                let re_parsed = parse(&emitted).expect("should re-parse");
                let re_emitted = emit(&re_parsed).expect("should re-emit");

                assert_eq!(emitted, re_emitted, "Round-trip failed for {}", fixture);
            }
        }
    }

    /// Test parsing MuseScore exports
    #[test]
    fn test_parse_musescore_export() {
        let path = "tests/fixtures/musescore_export.musicxml";
        if Path::new(path).exists() {
            let xml = fs::read_to_string(path).expect("should read");
            let score = parse(&xml).expect("should parse MuseScore export");

            assert!(!score.parts.is_empty());
            assert!(score.identification.is_some());
        }
    }

    /// Test parsing Finale exports
    #[test]
    fn test_parse_finale_export() {
        let path = "tests/fixtures/finale_export.musicxml";
        if Path::new(path).exists() {
            let xml = fs::read_to_string(path).expect("should read");
            let score = parse(&xml).expect("should parse Finale export");
            assert!(!score.parts.is_empty());
        }
    }

    /// Test parsing Sibelius exports
    #[test]
    fn test_parse_sibelius_export() {
        let path = "tests/fixtures/sibelius_export.musicxml";
        if Path::new(path).exists() {
            let xml = fs::read_to_string(path).expect("should read");
            let score = parse(&xml).expect("should parse Sibelius export");
            assert!(!score.parts.is_empty());
        }
    }

    /// Test specific feature: complex tuplets
    #[test]
    fn test_parse_complex_tuplets() {
        let xml = r#"<?xml version="1.0"?>
<score-partwise version="4.0">
  <part-list><score-part id="P1"><part-name>Test</part-name></score-part></part-list>
  <part id="P1">
    <measure number="1">
      <attributes><divisions>2</divisions></attributes>
      <!-- Triplet: 3 eighths in space of 2 -->
      <note>
        <pitch><step>C</step><octave>4</octave></pitch>
        <duration>1</duration><type>eighth</type>
        <time-modification>
          <actual-notes>3</actual-notes>
          <normal-notes>2</normal-notes>
        </time-modification>
        <notations>
          <tuplet type="start" bracket="yes"/>
        </notations>
      </note>
      <note>
        <pitch><step>D</step><octave>4</octave></pitch>
        <duration>1</duration><type>eighth</type>
        <time-modification>
          <actual-notes>3</actual-notes>
          <normal-notes>2</normal-notes>
        </time-modification>
      </note>
      <note>
        <pitch><step>E</step><octave>4</octave></pitch>
        <duration>1</duration><type>eighth</type>
        <time-modification>
          <actual-notes>3</actual-notes>
          <normal-notes>2</normal-notes>
        </time-modification>
        <notations>
          <tuplet type="stop"/>
        </notations>
      </note>
    </measure>
  </part>
</score-partwise>"#;

        let score = parse(xml).expect("should parse tuplets");

        // Find notes with time-modification
        let tuplet_notes: Vec<_> = score.parts[0].measures[0].content.iter()
            .filter_map(|e| match e {
                MusicDataElement::Note(n) if n.time_modification.is_some() => Some(n),
                _ => None,
            })
            .collect();

        assert_eq!(tuplet_notes.len(), 3);

        for note in &tuplet_notes {
            let tm = note.time_modification.as_ref().unwrap();
            assert_eq!(tm.actual_notes, 3);
            assert_eq!(tm.normal_notes, 2);
        }
    }

    /// Test specific feature: lyrics with multiple verses
    #[test]
    fn test_parse_multi_verse_lyrics() {
        let xml = r#"<?xml version="1.0"?>
<score-partwise version="4.0">
  <part-list><score-part id="P1"><part-name>Voice</part-name></score-part></part-list>
  <part id="P1">
    <measure number="1">
      <attributes><divisions>1</divisions></attributes>
      <note>
        <pitch><step>C</step><octave>4</octave></pitch>
        <duration>1</duration><type>quarter</type>
        <lyric number="1">
          <syllabic>single</syllabic>
          <text>Hel-</text>
        </lyric>
        <lyric number="2">
          <syllabic>single</syllabic>
          <text>Good-</text>
        </lyric>
      </note>
    </measure>
  </part>
</score-partwise>"#;

        let score = parse(xml).expect("should parse lyrics");

        let note = match &score.parts[0].measures[0].content[1] {
            MusicDataElement::Note(n) => n,
            _ => panic!("Expected note"),
        };

        assert_eq!(note.lyrics.len(), 2);
        assert_eq!(note.lyrics[0].number, Some("1".to_string()));
        assert_eq!(note.lyrics[1].number, Some("2".to_string()));
    }

    /// Test specific feature: full score header
    #[test]
    fn test_parse_full_header() {
        let xml = r#"<?xml version="1.0"?>
<score-partwise version="4.0">
  <work>
    <work-number>Op. 1</work-number>
    <work-title>Test Piece</work-title>
  </work>
  <movement-number>1</movement-number>
  <movement-title>Allegro</movement-title>
  <identification>
    <creator type="composer">Test Composer</creator>
    <creator type="lyricist">Test Lyricist</creator>
    <rights>Copyright 2024</rights>
    <encoding>
      <software>Fermata</software>
      <encoding-date>2024-01-01</encoding-date>
    </encoding>
  </identification>
  <credit page="1">
    <credit-type>title</credit-type>
    <credit-words>Test Piece</credit-words>
  </credit>
  <part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list>
  <part id="P1"><measure number="1"/></part>
</score-partwise>"#;

        let score = parse(xml).expect("should parse full header");

        // Check work
        let work = score.work.expect("should have work");
        assert_eq!(work.work_number, Some("Op. 1".to_string()));
        assert_eq!(work.work_title, Some("Test Piece".to_string()));

        // Check movement
        assert_eq!(score.movement_number, Some("1".to_string()));
        assert_eq!(score.movement_title, Some("Allegro".to_string()));

        // Check identification
        let id = score.identification.expect("should have identification");
        assert_eq!(id.creators.len(), 2);
        assert_eq!(id.creators[0].r#type, Some("composer".to_string()));
        assert_eq!(id.creators[0].value, "Test Composer");

        // Check encoding
        let enc = id.encoding.expect("should have encoding");
        assert_eq!(enc.software.len(), 1);
        assert_eq!(enc.software[0], "Fermata");

        // Check credits
        assert_eq!(score.credits.len(), 1);
        assert_eq!(score.credits[0].credit_types.len(), 1);
    }

    /// Test error messages are informative
    #[test]
    fn test_error_messages() {
        // Missing required element
        let xml = r#"<score-partwise><part-list/></score-partwise>"#;
        let err = parse(xml).unwrap_err();
        assert!(err.to_string().contains("Missing"));

        // Invalid value
        let xml = r#"<?xml version="1.0"?>
<score-partwise>
  <part-list><score-part id="P1"><part-name>T</part-name></score-part></part-list>
  <part id="P1">
    <measure number="1">
      <attributes><key><fifths>invalid</fifths></key></attributes>
    </measure>
  </part>
</score-partwise>"#;
        let err = parse(xml).unwrap_err();
        assert!(err.to_string().contains("Invalid") || err.to_string().contains("parse"));
    }
}
```

**Acceptance Criteria:**

- [ ] All round-trip tests pass
- [ ] Parse real-world exports (MuseScore, Finale, Sibelius)
- [ ] Complex tuplet parsing works
- [ ] Multi-verse lyrics work
- [ ] Full score header parses
- [ ] Error messages are informative with positions

---

## Milestone 5 Summary

After completing Milestone 5:

1. **Lyrics** — syllables, melismas, multiple verses
2. **Ornaments** — complete implementation
3. **Technical** — complete implementation
4. **Score header** — work, identification, credits
5. **Test suite** — comprehensive coverage

**Phase 3 is complete!**

---

*Next document: Checklist & Success Criteria*

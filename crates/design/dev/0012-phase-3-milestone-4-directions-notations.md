# Phase 3, Milestone 4: Directions & Notations

> **Document Series:** 5 of 7
> **Tasks:** 4.1–4.5
> **Focus:** Dynamics, articulations, slurs, ties, expression markings

---

## Overview

This milestone adds support for expressive elements:

- `<direction>` container with dynamics, wedges, metronome, text
- `<notations>` with ties, slurs, articulations, ornaments, fermatas

After this milestone, you can parse fully expressive scores.

---

## Task 4.1: Implement Direction Container Parsing

### Element Structure

```xml
<direction placement="below">
  <direction-type>
    <dynamics><ff/></dynamics>
  </direction-type>
  <voice>1</voice>
  <staff>1</staff>
</direction>
```

### Implementation

```rust
fn parse_direction(reader: &mut XmlReader, start: &BytesStart) -> Result<Direction, ParseError> {
    let placement = reader.get_optional_attr(start, "placement")
        .and_then(|s| parse_above_below(&s).ok());
    let directive = reader.get_optional_attr(start, "directive")
        .and_then(|s| parse_yes_no(&s).ok());
    let id = reader.get_optional_attr(start, "id");

    let mut direction_types = Vec::new();
    let mut offset = None;
    let mut voice = None;
    let mut staff = None;
    let mut sound = None;

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"direction-type" => direction_types.push(parse_direction_type(reader)?),
                    b"offset" => offset = Some(parse_offset(reader, &e)?),
                    b"voice" => voice = Some(reader.read_text()?),
                    b"staff" => staff = Some(reader.read_text_as()?),
                    b"sound" => sound = Some(parse_sound(reader, &e)?),
                    _ => reader.skip_element()?,
                }
            }
            Event::End(e) if e.name().as_ref() == b"direction" => break,
            _ => {}
        }
    }

    Ok(Direction {
        direction_types, offset, editorial_voice: None,
        voice, staff, sound, placement, directive, id,
    })
}

fn parse_direction_type(reader: &mut XmlReader) -> Result<DirectionType, ParseError> {
    let mut content = None;

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                content = Some(match e.name().as_ref() {
                    b"dynamics" => DirectionTypeContent::Dynamics(vec![parse_dynamics(reader, &e)?]),
                    b"wedge" => DirectionTypeContent::Wedge(parse_wedge(reader, &e)?),
                    b"words" => DirectionTypeContent::Words(vec![parse_words(reader, &e)?]),
                    b"metronome" => DirectionTypeContent::Metronome(parse_metronome(reader, &e)?),
                    b"rehearsal" => DirectionTypeContent::Rehearsal(vec![parse_rehearsal(reader, &e)?]),
                    b"segno" => DirectionTypeContent::Segno(vec![parse_segno(reader, &e)?]),
                    b"coda" => DirectionTypeContent::Coda(vec![parse_coda(reader, &e)?]),
                    b"pedal" => DirectionTypeContent::Pedal(parse_pedal(reader, &e)?),
                    b"octave-shift" => DirectionTypeContent::OctaveShift(parse_octave_shift(reader, &e)?),
                    _ => { reader.skip_element()?; continue; }
                });
            }
            Event::Empty(e) => {
                content = Some(match e.name().as_ref() {
                    b"wedge" => DirectionTypeContent::Wedge(parse_wedge_from_empty(&e)?),
                    b"segno" => DirectionTypeContent::Segno(vec![parse_segno_from_empty(&e)?]),
                    b"coda" => DirectionTypeContent::Coda(vec![parse_coda_from_empty(&e)?]),
                    _ => continue,
                });
            }
            Event::End(e) if e.name().as_ref() == b"direction-type" => break,
            _ => {}
        }
    }

    Ok(DirectionType { content: content.unwrap_or(DirectionTypeContent::Words(vec![])), id: None })
}
```

**Acceptance Criteria:**

- [ ] Parse direction with placement attribute
- [ ] Parse direction-type children (dynamics, wedge, words, metronome)
- [ ] Parse offset, voice, staff elements

---

## Task 4.2: Implement Dynamics Parsing

```rust
fn parse_dynamics(reader: &mut XmlReader, start: &BytesStart) -> Result<Dynamics, ParseError> {
    let placement = reader.get_optional_attr(start, "placement")
        .and_then(|s| parse_above_below(&s).ok());
    let id = reader.get_optional_attr(start, "id");

    let mut content = Vec::new();

    loop {
        match reader.next_event()? {
            Event::Start(e) | Event::Empty(e) => {
                let dyn_content = match e.name().as_ref() {
                    b"p" => DynamicsContent::P,
                    b"pp" => DynamicsContent::Pp,
                    b"ppp" => DynamicsContent::Ppp,
                    b"pppp" => DynamicsContent::Pppp,
                    b"ppppp" => DynamicsContent::Ppppp,
                    b"pppppp" => DynamicsContent::Pppppp,
                    b"f" => DynamicsContent::F,
                    b"ff" => DynamicsContent::Ff,
                    b"fff" => DynamicsContent::Fff,
                    b"ffff" => DynamicsContent::Ffff,
                    b"fffff" => DynamicsContent::Fffff,
                    b"ffffff" => DynamicsContent::Ffffff,
                    b"mp" => DynamicsContent::Mp,
                    b"mf" => DynamicsContent::Mf,
                    b"sf" => DynamicsContent::Sf,
                    b"sfp" => DynamicsContent::Sfp,
                    b"sfpp" => DynamicsContent::Sfpp,
                    b"fp" => DynamicsContent::Fp,
                    b"rf" => DynamicsContent::Rf,
                    b"rfz" => DynamicsContent::Rfz,
                    b"sfz" => DynamicsContent::Sfz,
                    b"sffz" => DynamicsContent::Sffz,
                    b"fz" => DynamicsContent::Fz,
                    b"n" => DynamicsContent::N,
                    b"pf" => DynamicsContent::Pf,
                    b"sfzp" => DynamicsContent::Sfzp,
                    b"other-dynamics" => DynamicsContent::OtherDynamics(reader.read_text()?),
                    _ => continue,
                };
                content.push(dyn_content);
            }
            Event::End(e) if e.name().as_ref() == b"dynamics" => break,
            _ => {}
        }
    }

    Ok(Dynamics {
        content, print_style: None, placement,
        text_decoration: None, enclosure: None, id,
    })
}
```

**Acceptance Criteria:**

- [ ] Parse all standard dynamics (p, pp, f, ff, mp, mf, etc.)
- [ ] Parse compound dynamics (sfz, sfp, fp, etc.)
- [ ] Parse other-dynamics with custom text

---

## Task 4.3: Implement Wedge and Metronome

```rust
fn parse_wedge(reader: &mut XmlReader, start: &BytesStart) -> Result<Wedge, ParseError> {
    let r#type = parse_wedge_type(&reader.get_attr(start, "type")?)?;
    let number = reader.get_optional_attr_as(start, "number");
    let spread = reader.get_optional_attr_as(start, "spread");
    let niente = reader.get_optional_attr(start, "niente").and_then(|s| parse_yes_no(&s).ok());
    let line_type = reader.get_optional_attr(start, "line-type").and_then(|s| parse_line_type(&s).ok());
    let id = reader.get_optional_attr(start, "id");

    reader.skip_element()?;

    Ok(Wedge { r#type, number, spread, niente, line_type, dashed_formatting: None, position: None, color: None, id })
}

fn parse_wedge_from_empty(start: &BytesStart) -> Result<Wedge, ParseError> {
    let r#type = parse_wedge_type(&get_attr(start, "type", 0)?)?;
    Ok(Wedge {
        r#type,
        number: get_optional_attr(start, "number").and_then(|s| s.parse().ok()),
        spread: get_optional_attr(start, "spread").and_then(|s| s.parse().ok()),
        niente: get_optional_attr(start, "niente").and_then(|s| parse_yes_no(&s).ok()),
        line_type: get_optional_attr(start, "line-type").and_then(|s| parse_line_type(&s).ok()),
        dashed_formatting: None, position: None, color: None,
        id: get_optional_attr(start, "id"),
    })
}

fn parse_metronome(reader: &mut XmlReader, start: &BytesStart) -> Result<Metronome, ParseError> {
    let parentheses = reader.get_optional_attr(start, "parentheses").and_then(|s| parse_yes_no(&s).ok());
    let id = reader.get_optional_attr(start, "id");

    let mut beat_unit = None;
    let mut beat_unit_dots = 0;
    let mut per_minute = None;

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"beat-unit" => beat_unit = Some(parse_note_type_value(&reader.read_text()?)?),
                    b"beat-unit-dot" => { beat_unit_dots += 1; reader.skip_element()?; }
                    b"per-minute" => per_minute = Some(PerMinute { value: reader.read_text()?, font: None }),
                    _ => reader.skip_element()?,
                }
            }
            Event::Empty(e) if e.name().as_ref() == b"beat-unit-dot" => beat_unit_dots += 1,
            Event::End(e) if e.name().as_ref() == b"metronome" => break,
            _ => {}
        }
    }

    let content = MetronomeContent::BeatUnit {
        beat_unit: beat_unit.unwrap_or(NoteTypeValue::Quarter),
        beat_unit_dots,
        beat_unit_tied: vec![],
        second_beat: None,
        per_minute,
    };

    Ok(Metronome { content, parentheses, print_style: None, print_object: None, justify: None, id })
}
```

**Acceptance Criteria:**

- [ ] Parse wedge (crescendo, diminuendo, stop, continue)
- [ ] Parse metronome with beat-unit and per-minute
- [ ] Handle dotted beat units

---

## Task 4.4: Implement Notations Container

```rust
fn parse_notations(reader: &mut XmlReader) -> Result<Notations, ParseError> {
    let mut content = Vec::new();

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                let notation = match e.name().as_ref() {
                    b"tied" => NotationContent::Tied(parse_tied(reader, &e)?),
                    b"slur" => NotationContent::Slur(parse_slur(reader, &e)?),
                    b"tuplet" => NotationContent::Tuplet(parse_tuplet(reader, &e)?),
                    b"ornaments" => NotationContent::Ornaments(parse_ornaments(reader)?),
                    b"technical" => NotationContent::Technical(parse_technical(reader)?),
                    b"articulations" => NotationContent::Articulations(parse_articulations(reader)?),
                    b"dynamics" => NotationContent::Dynamics(parse_dynamics(reader, &e)?),
                    b"fermata" => NotationContent::Fermata(parse_fermata(reader, &e)?),
                    b"arpeggiate" => NotationContent::Arpeggiate(parse_arpeggiate(reader, &e)?),
                    _ => { reader.skip_element()?; continue; }
                };
                content.push(notation);
            }
            Event::Empty(e) => {
                let notation = match e.name().as_ref() {
                    b"tied" => NotationContent::Tied(parse_tied_from_empty(&e)?),
                    b"slur" => NotationContent::Slur(parse_slur_from_empty(&e)?),
                    b"fermata" => NotationContent::Fermata(parse_fermata_from_empty(&e)?),
                    _ => continue,
                };
                content.push(notation);
            }
            Event::End(e) if e.name().as_ref() == b"notations" => break,
            _ => {}
        }
    }

    Ok(Notations { editorial: None, content, print_object: None, id: None })
}

fn parse_tied(reader: &mut XmlReader, start: &BytesStart) -> Result<Tied, ParseError> {
    let r#type = parse_tied_type(&reader.get_attr(start, "type")?)?;
    let number = reader.get_optional_attr_as(start, "number");
    let line_type = reader.get_optional_attr(start, "line-type").and_then(|s| parse_line_type(&s).ok());
    let placement = reader.get_optional_attr(start, "placement").and_then(|s| parse_above_below(&s).ok());
    let orientation = reader.get_optional_attr(start, "orientation").and_then(|s| parse_over_under(&s).ok());
    let id = reader.get_optional_attr(start, "id");

    reader.skip_element()?;

    Ok(Tied { r#type, number, line_type, dashed_formatting: None, position: None, placement, orientation, bezier: None, color: None, id })
}

fn parse_tied_from_empty(start: &BytesStart) -> Result<Tied, ParseError> {
    Ok(Tied {
        r#type: parse_tied_type(&get_attr(start, "type", 0)?)?,
        number: get_optional_attr(start, "number").and_then(|s| s.parse().ok()),
        line_type: get_optional_attr(start, "line-type").and_then(|s| parse_line_type(&s).ok()),
        dashed_formatting: None, position: None,
        placement: get_optional_attr(start, "placement").and_then(|s| parse_above_below(&s).ok()),
        orientation: get_optional_attr(start, "orientation").and_then(|s| parse_over_under(&s).ok()),
        bezier: None, color: None,
        id: get_optional_attr(start, "id"),
    })
}

fn parse_slur(reader: &mut XmlReader, start: &BytesStart) -> Result<Slur, ParseError> {
    let r#type = parse_start_stop_continue(&reader.get_attr(start, "type")?)?;
    let number = reader.get_optional_attr_as(start, "number").unwrap_or(1);
    let line_type = reader.get_optional_attr(start, "line-type").and_then(|s| parse_line_type(&s).ok());
    let placement = reader.get_optional_attr(start, "placement").and_then(|s| parse_above_below(&s).ok());
    let orientation = reader.get_optional_attr(start, "orientation").and_then(|s| parse_over_under(&s).ok());
    let id = reader.get_optional_attr(start, "id");

    reader.skip_element()?;

    Ok(Slur { r#type, number, line_type, dashed_formatting: None, position: None, placement, orientation, bezier: None, color: None, id })
}

fn parse_slur_from_empty(start: &BytesStart) -> Result<Slur, ParseError> {
    Ok(Slur {
        r#type: parse_start_stop_continue(&get_attr(start, "type", 0)?)?,
        number: get_optional_attr(start, "number").and_then(|s| s.parse().ok()).unwrap_or(1),
        line_type: get_optional_attr(start, "line-type").and_then(|s| parse_line_type(&s).ok()),
        dashed_formatting: None, position: None,
        placement: get_optional_attr(start, "placement").and_then(|s| parse_above_below(&s).ok()),
        orientation: get_optional_attr(start, "orientation").and_then(|s| parse_over_under(&s).ok()),
        bezier: None, color: None,
        id: get_optional_attr(start, "id"),
    })
}

fn parse_tuplet(reader: &mut XmlReader, start: &BytesStart) -> Result<Tuplet, ParseError> {
    let r#type = parse_start_stop(&reader.get_attr(start, "type")?)?;
    let number = reader.get_optional_attr_as(start, "number");
    let bracket = reader.get_optional_attr(start, "bracket").and_then(|s| parse_yes_no(&s).ok());
    let show_number = reader.get_optional_attr(start, "show-number"); // TODO: parse ShowTuplet
    let show_type = reader.get_optional_attr(start, "show-type");
    let id = reader.get_optional_attr(start, "id");

    // Parse optional actual/normal notes display
    let mut tuplet_actual = None;
    let mut tuplet_normal = None;

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"tuplet-actual" => tuplet_actual = Some(parse_tuplet_portion(reader)?),
                    b"tuplet-normal" => tuplet_normal = Some(parse_tuplet_portion(reader)?),
                    _ => reader.skip_element()?,
                }
            }
            Event::End(e) if e.name().as_ref() == b"tuplet" => break,
            _ => {}
        }
    }

    Ok(Tuplet {
        r#type, number, bracket,
        show_number: None, show_type: None,
        line_shape: None, position: None, placement: None,
        tuplet_actual, tuplet_normal, id,
    })
}

fn parse_tuplet_portion(reader: &mut XmlReader) -> Result<TupletPortion, ParseError> {
    let mut tuplet_number = None;
    let mut tuplet_type = None;
    let mut tuplet_dots = Vec::new();

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"tuplet-number" => tuplet_number = Some(parse_tuplet_number(reader, &e)?),
                    b"tuplet-type" => tuplet_type = Some(parse_tuplet_type(reader, &e)?),
                    b"tuplet-dot" => tuplet_dots.push(parse_tuplet_dot(reader, &e)?),
                    _ => reader.skip_element()?,
                }
            }
            Event::End(_) => break,
            _ => {}
        }
    }

    Ok(TupletPortion { tuplet_number, tuplet_type, tuplet_dots })
}
```

**Acceptance Criteria:**

- [ ] Parse tied start/stop/continue
- [ ] Parse slur with number attribute
- [ ] Parse tuplet brackets and display

---

## Task 4.5: Implement Articulations and Fermata

```rust
fn parse_articulations(reader: &mut XmlReader) -> Result<Articulations, ParseError> {
    let mut content = Vec::new();

    loop {
        match reader.next_event()? {
            Event::Start(e) | Event::Empty(e) => {
                let art = match e.name().as_ref() {
                    b"accent" => ArticulationContent::Accent(parse_empty_placement(&e)),
                    b"strong-accent" => ArticulationContent::StrongAccent(parse_strong_accent(&e)),
                    b"staccato" => ArticulationContent::Staccato(parse_empty_placement(&e)),
                    b"tenuto" => ArticulationContent::Tenuto(parse_empty_placement(&e)),
                    b"detached-legato" => ArticulationContent::DetachedLegato(parse_empty_placement(&e)),
                    b"staccatissimo" => ArticulationContent::Staccatissimo(parse_empty_placement(&e)),
                    b"spiccato" => ArticulationContent::Spiccato(parse_empty_placement(&e)),
                    b"scoop" => ArticulationContent::Scoop(parse_empty_line(&e)),
                    b"plop" => ArticulationContent::Plop(parse_empty_line(&e)),
                    b"doit" => ArticulationContent::Doit(parse_empty_line(&e)),
                    b"falloff" => ArticulationContent::Falloff(parse_empty_line(&e)),
                    b"breath-mark" => ArticulationContent::BreathMark(parse_breath_mark(reader, &e)?),
                    b"caesura" => ArticulationContent::Caesura(parse_caesura(reader, &e)?),
                    b"stress" => ArticulationContent::Stress(parse_empty_placement(&e)),
                    b"unstress" => ArticulationContent::Unstress(parse_empty_placement(&e)),
                    _ => continue,
                };
                content.push(art);
            }
            Event::End(e) if e.name().as_ref() == b"articulations" => break,
            _ => {}
        }
    }

    Ok(Articulations { content, id: None })
}

fn parse_empty_placement(start: &BytesStart) -> EmptyPlacement {
    EmptyPlacement {
        print_style: None,
        placement: get_optional_attr(start, "placement").and_then(|s| parse_above_below(&s).ok()),
    }
}

fn parse_strong_accent(start: &BytesStart) -> StrongAccent {
    StrongAccent {
        r#type: get_optional_attr(start, "type").and_then(|s| parse_up_down(&s).ok()),
        print_style: None,
        placement: get_optional_attr(start, "placement").and_then(|s| parse_above_below(&s).ok()),
    }
}

fn parse_fermata(reader: &mut XmlReader, start: &BytesStart) -> Result<Fermata, ParseError> {
    let r#type = reader.get_optional_attr(start, "type")
        .and_then(|s| parse_upright_inverted(&s).ok());
    let id = reader.get_optional_attr(start, "id");

    let shape_str = reader.read_text().unwrap_or_default();
    let shape = parse_fermata_shape(&shape_str)?;

    Ok(Fermata { shape, r#type, print_style: None, id })
}

fn parse_fermata_from_empty(start: &BytesStart) -> Result<Fermata, ParseError> {
    Ok(Fermata {
        shape: FermataShape::Normal,
        r#type: get_optional_attr(start, "type").and_then(|s| parse_upright_inverted(&s).ok()),
        print_style: None,
        id: get_optional_attr(start, "id"),
    })
}

fn parse_ornaments(reader: &mut XmlReader) -> Result<Ornaments, ParseError> {
    let mut content = Vec::new();

    loop {
        match reader.next_event()? {
            Event::Start(e) | Event::Empty(e) => {
                let orn = match e.name().as_ref() {
                    b"trill-mark" => OrnamentContent::TrillMark(parse_empty_trill_sound(&e)),
                    b"turn" => OrnamentContent::Turn(parse_horizontal_turn(&e)),
                    b"delayed-turn" => OrnamentContent::DelayedTurn(parse_horizontal_turn(&e)),
                    b"inverted-turn" => OrnamentContent::InvertedTurn(parse_horizontal_turn(&e)),
                    b"delayed-inverted-turn" => OrnamentContent::DelayedInvertedTurn(parse_horizontal_turn(&e)),
                    b"vertical-turn" => OrnamentContent::VerticalTurn(parse_empty_trill_sound(&e)),
                    b"inverted-vertical-turn" => OrnamentContent::InvertedVerticalTurn(parse_empty_trill_sound(&e)),
                    b"shake" => OrnamentContent::Shake(parse_empty_trill_sound(&e)),
                    b"wavy-line" => OrnamentContent::WavyLine(parse_wavy_line_ornament(reader, &e)?),
                    b"mordent" => OrnamentContent::Mordent(parse_mordent(&e)),
                    b"inverted-mordent" => OrnamentContent::InvertedMordent(parse_mordent(&e)),
                    b"schleifer" => OrnamentContent::Schleifer(parse_empty_placement(&e)),
                    b"tremolo" => OrnamentContent::Tremolo(parse_tremolo(reader, &e)?),
                    b"haydn" => OrnamentContent::Haydn(parse_empty_trill_sound(&e)),
                    b"accidental-mark" => OrnamentContent::AccidentalMark(parse_accidental_mark(reader, &e)?),
                    _ => continue,
                };
                content.push(orn);
            }
            Event::End(e) if e.name().as_ref() == b"ornaments" => break,
            _ => {}
        }
    }

    Ok(Ornaments { content, id: None })
}

fn parse_tremolo(reader: &mut XmlReader, start: &BytesStart) -> Result<Tremolo, ParseError> {
    let r#type = reader.get_optional_attr(start, "type")
        .and_then(|s| parse_tremolo_type(&s).ok())
        .unwrap_or(TremoloType::Single);

    let marks_str = reader.read_text().unwrap_or_default();
    let marks = marks_str.trim().parse().unwrap_or(3);

    Ok(Tremolo {
        marks,
        r#type,
        print_style: None,
        placement: None,
        smufl: None,
    })
}

fn parse_technical(reader: &mut XmlReader) -> Result<Technical, ParseError> {
    let mut content = Vec::new();

    loop {
        match reader.next_event()? {
            Event::Start(e) | Event::Empty(e) => {
                let tech = match e.name().as_ref() {
                    b"up-bow" => TechnicalContent::UpBow(parse_empty_placement(&e)),
                    b"down-bow" => TechnicalContent::DownBow(parse_empty_placement(&e)),
                    b"harmonic" => TechnicalContent::Harmonic(parse_harmonic(reader, &e)?),
                    b"open-string" => TechnicalContent::OpenString(parse_empty_placement(&e)),
                    b"thumb-position" => TechnicalContent::ThumbPosition(parse_empty_placement(&e)),
                    b"fingering" => TechnicalContent::Fingering(parse_fingering(reader, &e)?),
                    b"pluck" => TechnicalContent::Pluck(parse_pluck(reader, &e)?),
                    b"double-tongue" => TechnicalContent::DoubleTongue(parse_empty_placement(&e)),
                    b"triple-tongue" => TechnicalContent::TripleTongue(parse_empty_placement(&e)),
                    b"stopped" => TechnicalContent::Stopped(parse_empty_placement(&e)),
                    b"snap-pizzicato" => TechnicalContent::SnapPizzicato(parse_empty_placement(&e)),
                    b"fret" => TechnicalContent::Fret(parse_fret(reader, &e)?),
                    b"string" => TechnicalContent::String(parse_string_number(reader, &e)?),
                    b"hammer-on" => TechnicalContent::HammerOn(parse_hammer_on_pull_off(reader, &e)?),
                    b"pull-off" => TechnicalContent::PullOff(parse_hammer_on_pull_off(reader, &e)?),
                    b"tap" => TechnicalContent::Tap(parse_tap(reader, &e)?),
                    _ => continue,
                };
                content.push(tech);
            }
            Event::End(e) if e.name().as_ref() == b"technical" => break,
            _ => {}
        }
    }

    Ok(Technical { content, id: None })
}

fn parse_fingering(reader: &mut XmlReader, start: &BytesStart) -> Result<Fingering, ParseError> {
    let substitution = reader.get_optional_attr(start, "substitution").and_then(|s| parse_yes_no(&s).ok());
    let alternate = reader.get_optional_attr(start, "alternate").and_then(|s| parse_yes_no(&s).ok());
    let placement = reader.get_optional_attr(start, "placement").and_then(|s| parse_above_below(&s).ok());

    let value = reader.read_text()?;

    Ok(Fingering { value, substitution, alternate, print_style: None, placement })
}
```

**Acceptance Criteria:**

- [ ] Parse all articulation types (staccato, accent, tenuto, etc.)
- [ ] Parse fermata shapes (normal, angled, square)
- [ ] Parse ornaments (trill, turn, mordent, tremolo)
- [ ] Parse technical marks (fingering, bowing, etc.)

---

## Milestone 4 Summary

After completing Milestone 4:

1. **Directions** — dynamics, wedges, metronome, text
2. **Notations** — tied, slur, tuplet containers
3. **Articulations** — staccato, accent, tenuto, etc.
4. **Ornaments** — trill, turn, mordent, tremolo
5. **Technical** — fingering, bowing marks

You can now parse fully expressive scores with dynamics and articulations.

---

*Next document: Milestone 5 — Extended Features*

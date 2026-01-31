# Fermata Phase 2: MusicXML Emitter — Implementation Plan

> **For:** Claude Code (Opus with Rust-SKILL.md agents)
> **Created:** 2026-01-31
> **Status:** Ready for execution

---

## Executive Summary

**Objective:** Implement `src/musicxml/emit.rs` that transforms the completed IR types into valid MusicXML 4.0 documents.

**Success Criteria:**

1. Generated MusicXML validates against the MusicXML 4.0 DTD
2. Output opens correctly in MuseScore 4
3. Round-trip fidelity: IR → XML → (future parser) → IR should be lossless
4. Clean, idiomatic Rust with comprehensive tests

**Estimated Scope:** ~20 implementation tasks, organized into 5 milestones

---

## Architecture Decision: XML Writer Strategy

### Recommended: `quick-xml` with Writer API

```toml
# Add to Cargo.toml
[dependencies]
quick-xml = "0.37"
```

**Rationale:**

- Well-maintained, fast, widely used
- `Writer` API provides streaming output (memory-efficient for large scores)
- Handles escaping, indentation, and encoding automatically
- Good error types for Result-based API

**Alternative considered:** Manual `fmt::Write` — more control but error-prone for complex documents.

---

## Module Structure

```
src/
├── ir/                    # ✅ Complete
│   └── (all existing modules)
├── musicxml/
│   ├── mod.rs             # Public API: emit(score) → Result<String>
│   ├── emit.rs            # Main emission logic
│   ├── writer.rs          # XmlWriter wrapper with helpers
│   └── divisions.rs       # Duration/divisions calculations
└── lib.rs                 # Re-export musicxml module
```

---

## Milestone 1: Foundation (Tasks 1.1–1.4)

### Task 1.1: Create Module Structure

Create the `musicxml` module scaffold:

```rust
// src/musicxml/mod.rs
mod emit;
mod writer;
mod divisions;

pub use emit::emit_score;

use crate::ir::ScorePartwise;

/// Emit a MusicXML document from a ScorePartwise IR.
/// Returns the complete XML string including declaration and DOCTYPE.
pub fn emit(score: &ScorePartwise) -> Result<String, EmitError> {
    emit::emit_score(score)
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmitError {
    XmlWrite(String),
    InvalidData(String),
}

impl std::fmt::Display for EmitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmitError::XmlWrite(msg) => write!(f, "XML write error: {}", msg),
            EmitError::InvalidData(msg) => write!(f, "Invalid IR data: {}", msg),
        }
    }
}

impl std::error::Error for EmitError {}
```

**Acceptance:** Module compiles, `emit()` is callable (can return placeholder).

---

### Task 1.2: Implement XmlWriter Helper

Create a wrapper that simplifies common patterns:

```rust
// src/musicxml/writer.rs
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use std::io::Cursor;

pub struct XmlWriter {
    writer: Writer<Cursor<Vec<u8>>>,
}

impl XmlWriter {
    pub fn new() -> Self {
        let writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);
        Self { writer }
    }

    /// Write XML declaration and DOCTYPE
    pub fn write_header(&mut self) -> Result<(), quick_xml::Error> {
        // <?xml version="1.0" encoding="UTF-8"?>
        self.writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;

        // DOCTYPE for MusicXML 4.0 partwise
        self.writer.get_mut().get_mut().extend_from_slice(
            b"\n<!DOCTYPE score-partwise PUBLIC \"-//Recordare//DTD MusicXML 4.0 Partwise//EN\" \"http://www.musicxml.org/dtds/partwise.dtd\">\n"
        );
        Ok(())
    }

    /// Start an element with no attributes
    pub fn start_element(&mut self, name: &str) -> Result<(), quick_xml::Error> {
        self.writer.write_event(Event::Start(BytesStart::new(name)))
    }

    /// Write a start tag with attributes from builder
    pub fn write_start(&mut self, builder: ElementBuilder) -> Result<(), quick_xml::Error> {
        self.writer.write_event(Event::Start(builder.into_bytes_start()))
    }

    /// End the current element
    pub fn end_element(&mut self, name: &str) -> Result<(), quick_xml::Error> {
        self.writer.write_event(Event::End(BytesEnd::new(name)))
    }

    /// Write an empty element <name/>
    pub fn empty_element(&mut self, name: &str) -> Result<(), quick_xml::Error> {
        self.writer.write_event(Event::Empty(BytesStart::new(name)))
    }

    /// Write an empty element with attributes
    pub fn empty_element_with_attrs(&mut self, builder: ElementBuilder) -> Result<(), quick_xml::Error> {
        self.writer.write_event(Event::Empty(builder.into_bytes_start()))
    }

    /// Write a simple element: <name>text</name>
    pub fn text_element(&mut self, name: &str, text: &str) -> Result<(), quick_xml::Error> {
        self.start_element(name)?;
        self.writer.write_event(Event::Text(BytesText::new(text)))?;
        self.end_element(name)
    }

    /// Write element only if value is Some
    pub fn optional_text_element<T: std::fmt::Display>(
        &mut self,
        name: &str,
        value: &Option<T>,
    ) -> Result<(), quick_xml::Error> {
        if let Some(v) = value {
            self.text_element(name, &v.to_string())?;
        }
        Ok(())
    }

    /// Write raw text content (for use inside open elements)
    pub fn write_text(&mut self, text: &str) -> Result<(), quick_xml::Error> {
        self.writer.write_event(Event::Text(BytesText::new(text)))
    }

    /// Consume and return the XML string
    pub fn into_string(self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.writer.into_inner().into_inner())
    }
}

/// Builder for elements with attributes
pub struct ElementBuilder {
    start: BytesStart<'static>,
}

impl ElementBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            start: BytesStart::new(name.to_string()),
        }
    }

    pub fn attr(mut self, key: &str, value: &str) -> Self {
        self.start.push_attribute((key, value));
        self
    }

    pub fn optional_attr<T: std::fmt::Display>(self, key: &str, value: &Option<T>) -> Self {
        match value {
            Some(v) => self.attr(key, &v.to_string()),
            None => self,
        }
    }

    pub fn into_bytes_start(self) -> BytesStart<'static> {
        self.start
    }
}
```

**Acceptance:** Unit tests pass for helper methods.

---

### Task 1.3: Implement Divisions Calculator

The `divisions` element defines how many units equal one quarter note. We need to compute the LCM of all duration denominators in a part.

```rust
// src/musicxml/divisions.rs
use crate::ir::{NoteTypeValue, TimeModification};

/// Standard divisions value that handles all common subdivisions.
/// 960 = LCM(1,2,3,4,5,6,7,8,9,10,12,15,16) — covers up to 16th note triplets and quintuplets
pub const STANDARD_DIVISIONS: u32 = 960;

/// Convert a note type to its duration in divisions (given divisions per quarter)
pub fn note_type_to_divisions(note_type: &NoteTypeValue, divisions: u32) -> u32 {
    match note_type {
        NoteTypeValue::Maxima => divisions * 32,
        NoteTypeValue::Long => divisions * 16,
        NoteTypeValue::Breve => divisions * 8,
        NoteTypeValue::Whole => divisions * 4,
        NoteTypeValue::Half => divisions * 2,
        NoteTypeValue::Quarter => divisions,
        NoteTypeValue::Eighth => divisions / 2,
        NoteTypeValue::N16th => divisions / 4,
        NoteTypeValue::N32nd => divisions / 8,
        NoteTypeValue::N64th => divisions / 16,
        NoteTypeValue::N128th => divisions / 32,
        NoteTypeValue::N256th => divisions / 64,
        NoteTypeValue::N512th => divisions / 128,
        NoteTypeValue::N1024th => divisions / 256,
    }
}

/// Apply dots to a base duration
/// Each dot adds half of the previous duration
pub fn apply_dots(base_duration: u32, num_dots: usize) -> u32 {
    let mut total = base_duration;
    let mut addition = base_duration;
    for _ in 0..num_dots {
        addition /= 2;
        total += addition;
    }
    total
}

/// Apply time modification (tuplet ratio) to a duration
pub fn apply_time_modification(duration: u32, time_mod: &TimeModification) -> u32 {
    // actual_notes in the time of normal_notes
    // e.g., 3:2 triplet means 3 notes in time of 2
    // So each note's duration = base_duration * normal / actual
    (duration * time_mod.normal_notes) / time_mod.actual_notes
}

/// Calculate duration for a note given divisions, type, dots, and time modification
pub fn calculate_duration(
    note_type: &NoteTypeValue,
    dots: usize,
    time_modification: Option<&TimeModification>,
    divisions: u32,
) -> u32 {
    let base = note_type_to_divisions(note_type, divisions);
    let dotted = apply_dots(base, dots);
    match time_modification {
        Some(tm) => apply_time_modification(dotted, tm),
        None => dotted,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quarter_note() {
        assert_eq!(note_type_to_divisions(&NoteTypeValue::Quarter, 960), 960);
    }

    #[test]
    fn test_half_note() {
        assert_eq!(note_type_to_divisions(&NoteTypeValue::Half, 960), 1920);
    }

    #[test]
    fn test_eighth_note() {
        assert_eq!(note_type_to_divisions(&NoteTypeValue::Eighth, 960), 480);
    }

    #[test]
    fn test_dotted_quarter() {
        let base = note_type_to_divisions(&NoteTypeValue::Quarter, 960);
        assert_eq!(apply_dots(base, 1), 1440);
    }

    #[test]
    fn test_triplet_quarter() {
        let tm = TimeModification {
            actual_notes: 3,
            normal_notes: 2,
            normal_type: None,
            normal_dots: 0,
        };
        let base = note_type_to_divisions(&NoteTypeValue::Quarter, 960);
        assert_eq!(apply_time_modification(base, &tm), 640);
    }
}
```

**Acceptance:** Unit tests verify duration calculations.

---

### Task 1.4: Minimal Score Emission (Skeleton Only)

Implement the top-level structure emission:

```rust
// src/musicxml/emit.rs
use crate::ir::*;
use crate::musicxml::writer::{XmlWriter, ElementBuilder};
use crate::musicxml::EmitError;

pub fn emit_score(score: &ScorePartwise) -> Result<String, EmitError> {
    let mut w = XmlWriter::new();

    w.write_header().map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // <score-partwise version="4.0">
    let mut root = ElementBuilder::new("score-partwise");
    if let Some(ref v) = score.version {
        root = root.attr("version", v);
    }
    w.write_start(root).map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // Score header elements (work, identification, defaults, credits, part-list)
    emit_score_header(&mut w, score)?;

    // Parts
    for part in &score.parts {
        emit_part(&mut w, part)?;
    }

    w.end_element("score-partwise").map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    w.into_string().map_err(|e| EmitError::XmlWrite(e.to_string()))
}

fn emit_score_header(w: &mut XmlWriter, score: &ScorePartwise) -> Result<(), EmitError> {
    // TODO: work, movement-number, movement-title, identification, defaults, credits
    emit_part_list(w, &score.part_list)?;
    Ok(())
}

fn emit_part_list(w: &mut XmlWriter, part_list: &PartList) -> Result<(), EmitError> {
    w.start_element("part-list").map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    for element in &part_list.content {
        match element {
            PartListElement::ScorePart(sp) => emit_score_part(w, sp)?,
            PartListElement::PartGroup(pg) => emit_part_group(w, pg)?,
        }
    }

    w.end_element("part-list").map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

fn emit_score_part(w: &mut XmlWriter, sp: &ScorePart) -> Result<(), EmitError> {
    let elem = ElementBuilder::new("score-part").attr("id", &sp.id);
    w.write_start(elem).map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // part-name is required
    w.text_element("part-name", &sp.part_name.value)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // TODO: part-name-display, part-abbreviation, group, score-instrument, midi-device, midi-instrument

    w.end_element("score-part").map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

fn emit_part_group(w: &mut XmlWriter, _pg: &PartGroup) -> Result<(), EmitError> {
    // TODO: implement part-group emission
    Ok(())
}

fn emit_part(w: &mut XmlWriter, part: &Part) -> Result<(), EmitError> {
    let elem = ElementBuilder::new("part").attr("id", &part.id);
    w.write_start(elem).map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    for measure in &part.measures {
        emit_measure(w, measure)?;
    }

    w.end_element("part").map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

fn emit_measure(w: &mut XmlWriter, measure: &Measure) -> Result<(), EmitError> {
    let elem = ElementBuilder::new("measure").attr("number", &measure.number);
    w.write_start(elem).map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    for element in &measure.content {
        emit_music_data(w, element)?;
    }

    w.end_element("measure").map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

fn emit_music_data(w: &mut XmlWriter, element: &MusicDataElement) -> Result<(), EmitError> {
    match element {
        MusicDataElement::Note(note) => emit_note(w, note),
        MusicDataElement::Backup(backup) => emit_backup(w, backup),
        MusicDataElement::Forward(forward) => emit_forward(w, forward),
        MusicDataElement::Direction(dir) => emit_direction(w, dir),
        MusicDataElement::Attributes(attrs) => emit_attributes(w, attrs),
        MusicDataElement::Harmony(harmony) => emit_harmony(w, harmony),
        MusicDataElement::FiguredBass(fb) => emit_figured_bass(w, fb),
        MusicDataElement::Print(print) => emit_print(w, print),
        MusicDataElement::Sound(sound) => emit_sound(w, sound),
        MusicDataElement::Listening(listening) => emit_listening(w, listening),
        MusicDataElement::Barline(barline) => emit_barline(w, barline),
        MusicDataElement::Grouping(grouping) => emit_grouping(w, grouping),
        MusicDataElement::Link(link) => emit_link(w, link),
        MusicDataElement::Bookmark(bookmark) => emit_bookmark(w, bookmark),
    }
}

// Stub implementations — to be completed in later tasks
fn emit_note(w: &mut XmlWriter, _note: &Note) -> Result<(), EmitError> { Ok(()) }
fn emit_backup(w: &mut XmlWriter, _backup: &Backup) -> Result<(), EmitError> { Ok(()) }
fn emit_forward(w: &mut XmlWriter, _forward: &Forward) -> Result<(), EmitError> { Ok(()) }
fn emit_direction(w: &mut XmlWriter, _dir: &Direction) -> Result<(), EmitError> { Ok(()) }
fn emit_attributes(w: &mut XmlWriter, _attrs: &Attributes) -> Result<(), EmitError> { Ok(()) }
fn emit_harmony(w: &mut XmlWriter, _harmony: &Harmony) -> Result<(), EmitError> { Ok(()) }
fn emit_figured_bass(w: &mut XmlWriter, _fb: &FiguredBass) -> Result<(), EmitError> { Ok(()) }
fn emit_print(w: &mut XmlWriter, _print: &Print) -> Result<(), EmitError> { Ok(()) }
fn emit_sound(w: &mut XmlWriter, _sound: &Sound) -> Result<(), EmitError> { Ok(()) }
fn emit_listening(w: &mut XmlWriter, _listening: &Listening) -> Result<(), EmitError> { Ok(()) }
fn emit_barline(w: &mut XmlWriter, _barline: &Barline) -> Result<(), EmitError> { Ok(()) }
fn emit_grouping(w: &mut XmlWriter, _grouping: &Grouping) -> Result<(), EmitError> { Ok(()) }
fn emit_link(w: &mut XmlWriter, _link: &Link) -> Result<(), EmitError> { Ok(()) }
fn emit_bookmark(w: &mut XmlWriter, _bookmark: &Bookmark) -> Result<(), EmitError> { Ok(()) }
```

**Acceptance:** Can emit an empty score with part-list skeleton.

---

## Milestone 2: Core Note & Attributes (Tasks 2.1–2.5)

### Task 2.1: Implement Attributes Emission

The `<attributes>` element contains key, time, clef, divisions, etc.

**Element Order (per XSD):**

1. `<footnote>`, `<level>` (editorial)
2. `<divisions>`
3. `<key>`*
4. `<time>`*
5. `<staves>`
6. `<part-symbol>`
7. `<instruments>`
8. `<clef>`*
9. `<staff-details>`*
10. `<transpose>`*
11. `<for-part>`*
12. `<directive>`*
13. `<measure-style>`*

**Implementation approach:**

- Emit elements in strict XSD order
- Handle multiple keys/times/clefs (for different staves)
- Convert enums to lowercase strings for XML values

**Acceptance:** Test emits valid attributes with key (C major), time (4/4), clef (treble).

---

### Task 2.2: Implement Core Note Emission

The `<note>` element is the most complex. Focus on core structure first.

**Element Order (per XSD):**

1. Grace OR Cue OR Full-note content (chord, pitch/rest/unpitched)
2. Duration (for regular and cue notes)
3. Tie
4. `<instrument>`*
5. Editorial voice (`<footnote>`, `<level>`, `<voice>`)
6. `<type>`
7. `<dot>`*
8. `<accidental>`
9. `<time-modification>`
10. `<stem>`
11. `<notehead>`
12. `<notehead-text>`
13. `<staff>`
14. `<beam>`* (0-8)
15. `<notations>`*
16. `<lyric>`*
17. `<play>`
18. `<listen>`

**Key helper functions needed:**

- `note_type_value_to_string()` - converts enum to XML string
- `emit_full_note()` - handles chord flag + pitch/rest/unpitched
- `emit_pitch()` - step, alter, octave

**Acceptance:** Test emits valid note with pitch C4, quarter duration.

---

### Task 2.3: Implement Tie, Grace, Accidental, Time Modification

**Tie:** Empty element with `type="start|stop|continue"` attribute

**Grace:** Empty element with optional attributes:

- `slash="yes|no"`
- `steal-time-previous`, `steal-time-following` (percentages)
- `make-time` (divisions)

**Accidental:** Element with value content + attributes:

- Values: sharp, flat, natural, double-sharp, double-flat, etc.
- Attributes: cautionary, editorial, parentheses, bracket

**Time Modification:** For tuplets

- `<actual-notes>` (required)
- `<normal-notes>` (required)
- `<normal-type>` (optional)
- `<normal-dot>`* (optional, multiple)

**Acceptance:** Test ties, grace notes, accidentals, and triplets.

---

### Task 2.4: Implement Beam and Stem

**Beam:** Element with value content + attributes

- Values: begin, continue, end, forward hook, backward hook
- `number` attribute (1-8, beam level)
- `repeater`, `fan` attributes

**Stem:** Element with value content

- Values: up, down, none, double
- Position attributes optional

**Notehead:** Element with value content + attributes

- Values: slash, triangle, diamond, square, cross, x, circle-x, normal, none, etc.
- `filled`, `parentheses` attributes

**Acceptance:** Test beaming patterns for eighth notes.

---

### Task 2.5: Write "Twinkle Twinkle" Integration Test

Create a complete test that emits "Twinkle Twinkle Little Star" (first phrase) and validates:

1. Output is well-formed XML
2. Opens in MuseScore without errors

The test should:

- Build a `ScorePartwise` programmatically
- Call `emit()` and verify structure
- Write output to `target/twinkle.musicxml` for manual inspection

**Acceptance:** XML opens in MuseScore, displays correct notes.

---

## Milestone 3: Voice, Barlines, Navigation (Tasks 3.1–3.4)

### Task 3.1: Implement Backup and Forward

**Backup:** Moves time position backward for multiple voices

- `<duration>` (required)
- Editorial elements optional

**Forward:** Moves time position forward

- `<duration>` (required)
- Editorial-voice, staff optional

**Acceptance:** Test two-voice measure with backup.

---

### Task 3.2: Implement Barline

**Barline:** Container for bar styling and navigation

- `location` attribute: left, right, middle
- Children: bar-style, wavy-line, segno, coda, fermata, ending, repeat

**Bar-style values:** regular, dotted, dashed, heavy, light-light, light-heavy, heavy-light, heavy-heavy, tick, short, none

**Ending:** Volta brackets

- `number` attribute (required)
- `type` attribute: start, stop, discontinue

**Repeat:** Repeat signs

- `direction` attribute: forward, backward
- `times` attribute (optional)

**Acceptance:** Test repeat barlines and volta brackets.

---

### Task 3.3: Write Multi-Voice Test

Create a test case with two voices (soprano + alto):

- Voice 1: C4 half, D4 half
- Voice 2: E3 half, F3 half
- Uses backup to return to start of measure for voice 2

**Acceptance:** Two voices display correctly in MuseScore.

---

### Task 3.4: Write Repeat/Volta Test

Test repeat signs and first/second endings:

- Measure 1: start repeat
- Measure 2: first ending
- Measure 3: second ending
- Correct barline placement

**Acceptance:** Repeats and volta brackets display correctly.

---

## Milestone 4: Directions and Notations (Tasks 4.1–4.6)

### Task 4.1: Implement Direction Container

**Direction:** Container for musical directions

- `placement` attribute: above, below
- `directive` attribute: yes/no
- Children: direction-type+, offset?, editorial-voice?, staff?, sound?

**Direction-type:** The actual direction content

- Many choices: rehearsal, segno, coda, words, dynamics, wedge, metronome, etc.

**Acceptance:** Compiles with direction structure.

---

### Task 4.2: Implement Dynamics

**Dynamics:** Container for dynamic markings

- Empty elements for standard dynamics: p, pp, ppp, f, ff, fff, mp, mf, sf, sfz, fp, etc.
- `<other-dynamics>` for text

**Acceptance:** Test dynamics markings display correctly.

---

### Task 4.3: Implement Wedge (Crescendo/Decrescendo)

**Wedge:** Hairpin dynamics

- `type` attribute: crescendo, diminuendo, stop, continue
- `number` attribute for multiple concurrent wedges
- `spread`, `niente`, `line-type` attributes

**Acceptance:** Test crescendo/decrescendo hairpins.

---

### Task 4.4: Implement Metronome

**Metronome:** Tempo markings

- `beat-unit` (note type)
- `beat-unit-dot`*
- `per-minute` (tempo value)
- Or complex patterns with `metronome-note`

**Acceptance:** Test tempo markings.

---

### Task 4.5: Implement Notations Container

**Notations:** Container for note-attached notations

- Children: tied, slur, tuplet, articulations, ornaments, technical, dynamics, fermata, arpeggiate, glissando, slide, etc.

**Tied:** Visual tie notation

- `type` attribute: start, stop, continue, let-ring
- `number`, `line-type`, bezier attributes

**Slur:** Slur/phrase marking

- `type` attribute: start, stop, continue
- `number` attribute (1-16)

**Acceptance:** Test tied notes and slurs.

---

### Task 4.6: Implement Articulations and Fermata

**Articulations:** Container

- accent, strong-accent, staccato, tenuto, detached-legato, staccatissimo, spiccato
- breath-mark, caesura
- scoop, plop, doit, falloff (jazz articulations)

**Fermata:** Pause/hold

- `type` attribute: upright, inverted
- Value: normal, angled, square, double-angled, double-square, double-dot, half-curve, curlew

**Acceptance:** Test staccato, accent, fermata markings.

---

## Milestone 5: Extended Features (Tasks 5.1–5.5)

### Task 5.1: Implement Lyrics

**Lyric:** Syllable attached to note

- `number` attribute (verse number)
- `name` attribute (part name)
- Children: syllabic?, text, elision*, extend?, end-line?, end-paragraph?

**Syllabic values:** single, begin, middle, end

**Extend:** Melisma line

**Acceptance:** Test lyric syllables with melisma.

---

### Task 5.2: Implement Ornaments

**Ornaments:** Container

- trill-mark, turn, delayed-turn, inverted-turn, shake, wavy-line
- mordent, inverted-mordent
- tremolo (with marks count)
- accidental-mark

**Acceptance:** Test trill marks and tremolos.

---

### Task 5.3: Implement Technical Marks

**Technical:** Container for instrument-specific marks

- up-bow, down-bow, harmonic, open-string, thumb-position
- fingering, pluck, string, fret
- hammer-on, pull-off, bend, tap
- heel, toe, fingernails

**Acceptance:** Test fingering and string numbers.

---

### Task 5.4: Implement Score Header Elements

Complete the score header (work, identification, defaults, credits):

**Work:** Work identification

- work-number, work-title, opus

**Identification:** Creator/rights metadata

- creator* (with type attribute: composer, lyricist, arranger, etc.)
- rights*
- encoding, source, relation

**Credit:** Title page text

**Acceptance:** Test score with title, composer, copyright.

---

### Task 5.5: Comprehensive Integration Test Suite

Create a test module with multiple real-world examples:

- Simple melody (single voice, basic attributes)
- Chord progression (multiple chord voicings)
- Piano grand staff (two staves, bass and treble)
- Tuplets and dots (triplets, dotted rhythms)
- Dynamics and articulations (full expression)
- Repeats and codas (navigation structure)
- Vocal with lyrics (syllables, melisma)
- Orchestral score (multiple parts, part groups)

**Acceptance:** All tests pass, output validated in MuseScore.

---

## Testing Strategy

### Unit Tests

- Each `emit_*` function should have isolated unit tests
- Test both happy path and edge cases (empty collections, None values)
- Verify exact XML output using string comparison

### Integration Tests

- End-to-end tests from IR to complete MusicXML document
- Write output to `target/` for manual inspection
- Compare against known-good reference files where available

### Validation Tests

- Validate against MusicXML 4.0 DTD (can use `xmllint`)
- Open in MuseScore, verify no warnings
- Round-trip test preparation (emit → parse → compare IR)

### Example Validation Helper

```rust
// tests/helpers.rs
use std::process::Command;

pub fn validate_musicxml(xml: &str) -> bool {
    let temp_file = std::env::temp_dir().join("test.musicxml");
    std::fs::write(&temp_file, xml).unwrap();

    let output = Command::new("xmllint")
        .arg("--valid")
        .arg("--noout")
        .arg(&temp_file)
        .output();

    match output {
        Ok(o) => o.status.success(),
        Err(_) => {
            eprintln!("Warning: xmllint not found, skipping validation");
            true
        }
    }
}
```

---

## Implementation Notes for Claude Code

### Error Handling Pattern

Use `?` with map_err consistently:

```rust
// Standard pattern
w.text_element("foo", "bar").map_err(|e| EmitError::XmlWrite(e.to_string()))?;

// Consider a helper macro for cleaner code
macro_rules! xml_err {
    ($expr:expr) => {
        $expr.map_err(|e| EmitError::XmlWrite(e.to_string()))?
    };
}
```

### Element Ordering is Critical

MusicXML's XSD defines strict element order. When implementing `emit_note`, for example, elements MUST appear in this order:

1. grace/cue/chord+pitch/rest
2. duration (for non-grace)
3. tie
4. instrument
5. editorial-voice
6. type
7. dot
8. accidental
9. time-modification
10. stem
11. notehead
12. staff
13. beam
14. notations
15. lyric
16. play
17. listen

The MusicXML spec defines this precisely. Incorrect ordering will cause validation failures.

### Helpful XSD Reference Patterns

```rust
// For xs:choice - use enum in IR, match in emit
enum Content { A(TypeA), B(TypeB) }

// For xs:sequence with optional elements - emit in order, skip None
if let Some(x) = &data.field { emit_x(w, x)?; }

// For xs:attribute with default - emit only if different from default
if attr != default_value { elem = elem.attr("key", &attr); }

// For repeating elements (*)
for item in &data.items { emit_item(w, item)?; }
```

### Repository Coordination

The IR types are already implemented in `src/ir/`. You'll need to:

1. Ensure all enum variants have Display or helper functions for XML values
2. Add any missing type conversions (e.g., `Step` → "C", "D", etc.)
3. Coordinate with existing code style and patterns
4. Check how existing enums are named (some may need adjustment for MusicXML output)

---

## Checklist for Claude Code

### Before Starting

- [ ] Clone repository: `git clone ssh://git@codeberg.org/oxur/fermata.git`
- [ ] Run existing tests: `cargo test`
- [ ] Read through `src/ir/` to understand existing types
- [ ] Add `quick-xml = "0.37"` to Cargo.toml

### Milestone 1 Checklist

- [ ] Task 1.1: Create `src/musicxml/mod.rs`, `emit.rs`, `writer.rs`, `divisions.rs`
- [ ] Task 1.2: Implement `XmlWriter` helper with tests
- [ ] Task 1.3: Implement divisions calculator with tests
- [ ] Task 1.4: Implement score skeleton (part-list, parts, measures)

### Milestone 2 Checklist

- [ ] Task 2.1: Implement attributes (key, time, clef, divisions)
- [ ] Task 2.2: Implement core note (pitch, duration, type, dots)
- [ ] Task 2.3: Implement tie, grace, accidental, time-modification
- [ ] Task 2.4: Implement beam and notehead
- [ ] Task 2.5: Write "Twinkle Twinkle" integration test

### Milestone 3 Checklist

- [ ] Task 3.1: Implement backup/forward
- [ ] Task 3.2: Implement barline (style, repeat, ending)
- [ ] Task 3.3: Write multi-voice test
- [ ] Task 3.4: Write repeat/volta test

### Milestone 4 Checklist

- [ ] Task 4.1: Implement direction container
- [ ] Task 4.2: Implement dynamics
- [ ] Task 4.3: Implement wedge (crescendo/decrescendo)
- [ ] Task 4.4: Implement metronome
- [ ] Task 4.5: Implement notations container
- [ ] Task 4.6: Implement articulations and fermata

### Milestone 5 Checklist

- [ ] Task 5.1: Implement lyrics
- [ ] Task 5.2: Implement ornaments
- [ ] Task 5.3: Implement technical marks
- [ ] Task 5.4: Implement score header (work, identification)
- [ ] Task 5.5: Write comprehensive integration test suite

### Final Validation

- [ ] All tests pass
- [ ] Output validates with `xmllint --valid`
- [ ] Open test files in MuseScore without errors
- [ ] Document any deferred features or known limitations
- [ ] Update bootstrap document with Phase 2 completion status

---

## Success Criteria

Phase 2 is complete when:

1. **Functional:** `fermata::musicxml::emit(&score)` returns valid MusicXML 4.0
2. **Correct:** Output displays correctly in MuseScore 4
3. **Tested:** Comprehensive unit and integration tests pass
4. **Documented:** Public API has doc comments
5. **Idiomatic:** Code follows Rust best practices (agents will ensure this)

---

## Reference Resources

- **MusicXML 4.0 Spec:** <https://www.w3.org/2021/06/musicxml40/>
- **MusicXML XSD:** <https://github.com/w3c/musicxml> (schema/ directory)
- **MusicXML Tutorial:** <https://www.w3.org/2021/06/musicxml40/tutorial/>
- **quick-xml docs:** <https://docs.rs/quick-xml/latest/quick_xml/>

---

*Document version: 2026-01-31*
*Target: Claude Code with Rust-SKILL.md agents*

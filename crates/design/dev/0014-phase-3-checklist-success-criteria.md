# Phase 3: Checklist & Success Criteria

> **Document Series:** 7 of 7
> **Purpose:** Final validation and handoff

---

## Pre-Flight Checklist

### Before Starting Phase 3

- [ ] **Phase 2 Complete:** All emitter tests pass
- [ ] **Repository Clean:** No uncommitted changes
- [ ] **Dependencies Ready:** `quick-xml = "0.37"` in Cargo.toml
- [ ] **Test Fixtures Available:** Sample MusicXML files in `tests/fixtures/`
- [ ] **Bootstrap Doc Reviewed:** Understand full project context

---

## Milestone Checklists

### Milestone 1: Foundation ✓

- [ ] **Task 1.1:** Module structure created
  - [ ] `src/musicxml/mod.rs` updated with `parse` export
  - [ ] `src/musicxml/parse.rs` created
  - [ ] `src/musicxml/reader.rs` created
  - [ ] `src/musicxml/values.rs` created
  - [ ] `ParseError` enum implemented with Display

- [ ] **Task 1.2:** XmlReader helper implemented
  - [ ] `new()` constructor
  - [ ] `next_event()` skips comments/PI
  - [ ] `read_text()` reads until end tag
  - [ ] `read_text_as<T>()` parses typed content
  - [ ] `skip_element()` handles nested elements
  - [ ] `get_attr()` and `get_optional_attr()`
  - [ ] `position()` for error reporting
  - [ ] Unit tests pass

- [ ] **Task 1.3:** Value parsers implemented
  - [ ] `parse_note_type_value()`
  - [ ] `parse_step()`
  - [ ] `parse_accidental_value()`
  - [ ] `parse_clef_sign()`
  - [ ] `parse_mode()`
  - [ ] `parse_yes_no()`
  - [ ] `parse_start_stop()` / `parse_start_stop_continue()`
  - [ ] `parse_beam_value()` / `parse_stem_value()`
  - [ ] `parse_bar_style()` / `parse_wedge_type()`
  - [ ] All unit tests pass

- [ ] **Task 1.4:** Top-level parsing skeleton
  - [ ] `parse_score()` handles XML declaration, DOCTYPE
  - [ ] Rejects `score-timewise` with helpful message
  - [ ] `parse_score_partwise()` extracts version
  - [ ] `parse_part_list()` with `score-part` and `part-group`
  - [ ] `parse_part()` and `parse_measure()` structure
  - [ ] Minimal score test passes

### Milestone 2: Core Note & Attributes ✓

- [ ] **Task 2.1:** Attributes parsing
  - [ ] `parse_attributes()` with all children
  - [ ] `parse_key()` for traditional keys
  - [ ] `parse_time()` with beats/beat-type and senza-misura
  - [ ] `parse_clef()` with sign, line, octave-change
  - [ ] Multiple keys/times/clefs supported

- [ ] **Task 2.2:** Core note parsing
  - [ ] `parse_note()` handles all three variants
  - [ ] `parse_pitch()` with step, alter, octave
  - [ ] `parse_rest()` with display position
  - [ ] `parse_unpitched()`
  - [ ] Chord flag (`<chord/>`) handled
  - [ ] Empty element forms handled

- [ ] **Task 2.3:** Supporting note elements
  - [ ] `parse_tie()` / `parse_tie_from_empty()`
  - [ ] `parse_grace()` / `parse_grace_from_empty()`
  - [ ] `parse_accidental()`
  - [ ] `parse_time_modification()`

- [ ] **Task 2.4:** Visual note elements
  - [ ] `parse_beam()`
  - [ ] `parse_stem()`
  - [ ] `parse_notehead()`
  - [ ] `parse_dot()` / `parse_dot_from_empty()`

- [ ] **Task 2.5:** Round-trip test
  - [ ] `emit → parse → emit` produces identical XML
  - [ ] IR equality preserved through round-trip
  - [ ] Double round-trip stable
  - [ ] Chords round-trip correctly
  - [ ] Accidentals round-trip correctly

### Milestone 3: Voice, Barlines, Navigation ✓

- [ ] **Task 3.1:** Backup and Forward
  - [ ] `parse_backup()` with duration
  - [ ] `parse_forward()` with duration, voice, staff

- [ ] **Task 3.2:** Barline parsing
  - [ ] `parse_barline()` with location attribute
  - [ ] `parse_bar_style_element()`
  - [ ] `parse_repeat()` / `parse_repeat_from_empty()`
  - [ ] `parse_ending()` with number, type, text
  - [ ] `parse_segno()` / `parse_coda()`

- [ ] **Task 3.3:** Multi-voice test passes
  - [ ] Two-voice measure parses correctly
  - [ ] Voice assignments preserved
  - [ ] Backup duration correct

- [ ] **Task 3.4:** Repeat/volta test passes
  - [ ] Forward/backward repeats parse
  - [ ] Volta endings parse (start, stop, discontinue)
  - [ ] Segno and coda markers parse

### Milestone 4: Directions & Notations ✓

- [ ] **Task 4.1:** Direction container
  - [ ] `parse_direction()` with placement
  - [ ] `parse_direction_type()` dispatcher
  - [ ] `parse_offset()`, voice, staff

- [ ] **Task 4.2:** Dynamics
  - [ ] `parse_dynamics()` all standard levels
  - [ ] Compound dynamics (sfz, fp, etc.)
  - [ ] `other-dynamics` with custom text

- [ ] **Task 4.3:** Wedge and Metronome
  - [ ] `parse_wedge()` with type, number, niente
  - [ ] `parse_metronome()` with beat-unit, per-minute
  - [ ] Dotted beat units handled

- [ ] **Task 4.4:** Notations container
  - [ ] `parse_notations()` dispatcher
  - [ ] `parse_tied()` start/stop/continue
  - [ ] `parse_slur()` with number
  - [ ] `parse_tuplet()` with bracket, show-number

- [ ] **Task 4.5:** Articulations and Fermata
  - [ ] `parse_articulations()` all types
  - [ ] `parse_fermata()` with shape
  - [ ] `parse_ornaments()` with trills, turns, mordents
  - [ ] `parse_technical()` with fingering, bowing

### Milestone 5: Extended Features ✓

- [ ] **Task 5.1:** Lyrics
  - [ ] `parse_lyric()` with syllabic, text
  - [ ] `parse_extend()` for melismas
  - [ ] Multiple verse numbers
  - [ ] Laughing/humming elements

- [ ] **Task 5.2:** Complete ornaments
  - [ ] All trill-sound attributes
  - [ ] Mordent long attribute
  - [ ] Tremolo with marks count

- [ ] **Task 5.3:** Complete technical
  - [ ] Harmonic (natural, artificial)
  - [ ] Fret and string numbers
  - [ ] Hammer-on, pull-off, tap

- [ ] **Task 5.4:** Score header
  - [ ] `parse_work()` with work-number, work-title
  - [ ] `parse_identification()` with creators, rights
  - [ ] `parse_encoding()` with software, date, supports
  - [ ] `parse_credit()` with credit-words

- [ ] **Task 5.5:** Integration test suite
  - [ ] All fixture round-trips pass
  - [ ] Real-world exports parse (MuseScore, Finale)
  - [ ] Complex tuplets test
  - [ ] Multi-verse lyrics test
  - [ ] Full header test
  - [ ] Error message tests

---

## Success Criteria

### Functional Requirements

| Requirement | Test | Status |
|-------------|------|--------|
| Parse valid MusicXML 4.0 partwise | `test_parse_minimal_score` | ☐ |
| Reject score-timewise with message | `test_reject_score_timewise` | ☐ |
| Round-trip fidelity | `test_round_trip_*` | ☐ |
| Parse all note variants | `test_parse_note_*` | ☐ |
| Parse multi-voice scores | `test_parse_two_voices` | ☐ |
| Parse repeat structures | `test_parse_repeats` | ☐ |
| Parse dynamics and directions | `test_parse_dynamics` | ☐ |
| Parse notations (slurs, ties) | `test_parse_notations` | ☐ |
| Parse lyrics | `test_parse_lyrics` | ☐ |
| Parse score header | `test_parse_full_header` | ☐ |

### Non-Functional Requirements

| Requirement | Metric | Status |
|-------------|--------|--------|
| Error messages include byte position | All ParseError variants | ☐ |
| Error messages are descriptive | Manual review | ☐ |
| Unknown elements skipped gracefully | No panics on unknown | ☐ |
| Memory efficient | No full DOM load | ☐ |
| Code is idiomatic Rust | Clippy clean | ☐ |
| Public API documented | `cargo doc` | ☐ |

### Compatibility

| Source | Test File | Parses | Round-trips |
|--------|-----------|--------|-------------|
| Hand-written | `twinkle.musicxml` | ☐ | ☐ |
| MuseScore 4 | `musescore_export.musicxml` | ☐ | ☐ |
| Finale | `finale_export.musicxml` | ☐ | ☐ |
| Sibelius | `sibelius_export.musicxml` | ☐ | ☐ |
| LilyPond | `lilypond_export.musicxml` | ☐ | ☐ |

---

## Final Validation Steps

### 1. Run All Tests

```bash
cargo test --all
```

All tests must pass.

### 2. Run Clippy

```bash
cargo clippy -- -D warnings
```

No warnings allowed.

### 3. Generate Documentation

```bash
cargo doc --open
```

Review `musicxml::parse` documentation.

### 4. Manual Validation

1. Create a test score in MuseScore
2. Export as MusicXML
3. Parse with Fermata: `let score = fermata::musicxml::parse(&xml)?`
4. Emit back: `let xml2 = fermata::musicxml::emit(&score)?`
5. Open `xml2` in MuseScore
6. Verify identical rendering

### 5. Update Bootstrap Document

Add to `0006-fermata-lisp-project-bootstrap-document.md`:

```markdown
## Phase 3: MusicXML Parser — COMPLETE

- `fermata::musicxml::parse(&xml)` converts MusicXML → IR
- Round-trip fidelity with Phase 2 emitter
- Comprehensive test coverage
- Compatible with MuseScore, Finale, Sibelius exports
```

---

## Handoff Notes

### Known Limitations

Document any known limitations:

- [ ] `score-timewise` not supported (intentional)
- [ ] Layout elements (`<defaults>`) stubbed (deferred)
- [ ] Some rarely-used elements may be skipped
- [ ] Non-traditional keys may have limited support

### Future Improvements

Potential enhancements for later:

- [ ] Lenient parsing mode with warnings
- [ ] Streaming parse for very large files
- [ ] Partial parse (specific parts/measures)
- [ ] Validation against XSD

### File Manifest

```
src/musicxml/
├── mod.rs           # Public API
├── emit.rs          # Phase 2: IR → XML
├── writer.rs        # Phase 2: XmlWriter helper
├── divisions.rs     # Phase 2: Duration calculations
├── parse.rs         # Phase 3: XML → IR
├── reader.rs        # Phase 3: XmlReader helper
└── values.rs        # Phase 3: String → enum parsers

tests/
├── emit_tests.rs    # Phase 2 tests
├── parse_tests.rs   # Phase 3 tests
├── round_trip.rs    # Combined tests
└── fixtures/
    ├── twinkle.musicxml
    ├── two_voices.musicxml
    ├── with_repeats.musicxml
    ├── with_dynamics.musicxml
    ├── with_lyrics.musicxml
    └── full_header.musicxml
```

---

## Sign-Off

**Phase 3 Complete When:**

- [ ] All milestone checklists complete
- [ ] All success criteria met
- [ ] All tests pass
- [ ] Documentation updated
- [ ] Bootstrap document updated
- [ ] Code reviewed and merged

---

*End of Phase 3 Documentation*

use super::*;
use crate::ir::attributes::TimeSymbol;

// === parse_score Tests ===

#[test]
fn test_parse_score_minimal() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
            <score-partwise version="4.0">
                <part-list>
                    <score-part id="P1">
                        <part-name>Piano</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert_eq!(score.version, Some("4.0".to_string()));
    assert_eq!(score.parts.len(), 1);
    assert_eq!(score.parts[0].id, "P1");
    assert_eq!(score.parts[0].measures.len(), 1);
    assert_eq!(score.parts[0].measures[0].number, "1");
}

#[test]
fn test_parse_score_with_doctype() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
            <!DOCTYPE score-partwise PUBLIC "-//Recordare//DTD MusicXML 4.0 Partwise//EN" "http://www.musicxml.org/dtds/partwise.dtd">
            <score-partwise version="4.0">
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert_eq!(score.version, Some("4.0".to_string()));
}

#[test]
fn test_parse_score_with_movement_title() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <movement-title>Symphony No. 5</movement-title>
                <part-list>
                    <score-part id="P1">
                        <part-name>Violin</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert_eq!(score.movement_title, Some("Symphony No. 5".to_string()));
}

#[test]
fn test_parse_score_missing_part_list() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let result = parse_score(xml);
    assert!(result.is_err());
    if let Err(ParseError::MissingElement {
        element, parent, ..
    }) = result
    {
        assert_eq!(element, "part-list");
        assert_eq!(parent, "score-partwise");
    } else {
        panic!("Expected MissingElement error");
    }
}

#[test]
fn test_parse_score_undefined_part_reference() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Piano</part-name>
                    </score-part>
                </part-list>
                <part id="P2">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let result = parse_score(xml);
    assert!(result.is_err());
    if let Err(ParseError::UndefinedReference {
        reference_type, id, ..
    }) = result
    {
        assert_eq!(reference_type, "part");
        assert_eq!(id, "P2");
    } else {
        panic!("Expected UndefinedReference error");
    }
}

#[test]
fn test_parse_score_timewise_not_supported() {
    let xml = r#"<?xml version="1.0"?>
            <score-timewise>
            </score-timewise>"#;

    let result = parse_score(xml);
    assert!(result.is_err());
}

// === Part List Tests ===

#[test]
fn test_parse_multiple_score_parts() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Violin I</part-name>
                    </score-part>
                    <score-part id="P2">
                        <part-name>Violin II</part-name>
                    </score-part>
                </part-list>
                <part id="P1"><measure number="1"/></part>
                <part id="P2"><measure number="1"/></part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert_eq!(score.part_list.content.len(), 2);
    assert_eq!(score.parts.len(), 2);
}

#[test]
fn test_parse_part_group() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <part-group type="start" number="1">
                        <group-name>Strings</group-name>
                        <group-symbol>bracket</group-symbol>
                    </part-group>
                    <score-part id="P1">
                        <part-name>Violin</part-name>
                    </score-part>
                    <part-group type="stop" number="1"/>
                </part-list>
                <part id="P1"><measure number="1"/></part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert_eq!(score.part_list.content.len(), 3);

    // First element should be part-group start
    if let PartListElement::PartGroup(pg) = &score.part_list.content[0] {
        assert_eq!(pg.r#type, crate::ir::common::StartStop::Start);
        assert_eq!(pg.number, Some("1".to_string()));
        assert!(pg.group_name.is_some());
    } else {
        panic!("Expected PartGroup");
    }
}

// === Measure Tests ===

#[test]
fn test_parse_measure_with_attributes() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1" implicit="yes" width="200">
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    let measure = &score.parts[0].measures[0];
    assert_eq!(measure.number, "1");
    assert_eq!(measure.implicit, Some(crate::ir::common::YesNo::Yes));
    assert_eq!(measure.width, Some(200.0));
}

#[test]
fn test_parse_multiple_measures() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                    <measure number="2"/>
                    <measure number="3"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert_eq!(score.parts[0].measures.len(), 3);
    assert_eq!(score.parts[0].measures[0].number, "1");
    assert_eq!(score.parts[0].measures[1].number, "2");
    assert_eq!(score.parts[0].measures[2].number, "3");
}

// === Backup/Forward Tests ===

#[test]
fn test_parse_backup() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <backup>
                            <duration>4</duration>
                        </backup>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert_eq!(score.parts[0].measures[0].content.len(), 1);
    if let crate::ir::measure::MusicDataElement::Backup(backup) =
        &score.parts[0].measures[0].content[0]
    {
        assert_eq!(backup.duration, 4);
    } else {
        panic!("Expected Backup");
    }
}

#[test]
fn test_parse_forward() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <forward>
                            <duration>8</duration>
                            <voice>2</voice>
                            <staff>1</staff>
                        </forward>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Forward(forward) =
        &score.parts[0].measures[0].content[0]
    {
        assert_eq!(forward.duration, 8);
        assert_eq!(forward.voice, Some("2".to_string()));
        assert_eq!(forward.staff, Some(1));
    } else {
        panic!("Expected Forward");
    }
}

// === Error Case Tests ===

#[test]
fn test_parse_empty_document() {
    let xml = "";
    let result = parse_score(xml);
    assert!(result.is_err());
}

#[test]
fn test_parse_missing_measure_number() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure>
                    </measure>
                </part>
            </score-partwise>"#;

    let result = parse_score(xml);
    assert!(result.is_err());
    if let Err(ParseError::MissingAttribute {
        attribute, element, ..
    }) = result
    {
        assert_eq!(attribute, "number");
        assert_eq!(element, "measure");
    } else {
        panic!("Expected MissingAttribute error");
    }
}

#[test]
fn test_parse_missing_score_part_id() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part>
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
            </score-partwise>"#;

    let result = parse_score(xml);
    assert!(result.is_err());
}

#[test]
fn test_parse_missing_part_name() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                    </score-part>
                </part-list>
            </score-partwise>"#;

    let result = parse_score(xml);
    assert!(result.is_err());
    if let Err(ParseError::MissingElement {
        element, parent, ..
    }) = result
    {
        assert_eq!(element, "part-name");
        assert_eq!(parent, "score-part");
    } else {
        panic!("Expected MissingElement error");
    }
}

// === Attributes Tests ===

#[test]
fn test_parse_attributes_divisions() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <attributes>
                            <divisions>4</divisions>
                        </attributes>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Attributes(attrs) =
        &score.parts[0].measures[0].content[0]
    {
        assert_eq!(attrs.divisions, Some(4));
    } else {
        panic!("Expected Attributes");
    }
}

#[test]
fn test_parse_attributes_key_signature() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <attributes>
                            <key>
                                <fifths>2</fifths>
                                <mode>major</mode>
                            </key>
                        </attributes>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Attributes(attrs) =
        &score.parts[0].measures[0].content[0]
    {
        assert_eq!(attrs.keys.len(), 1);
        if let KeyContent::Traditional(tk) = &attrs.keys[0].content {
            assert_eq!(tk.fifths, 2);
            assert_eq!(tk.mode, Some(Mode::Major));
        } else {
            panic!("Expected Traditional key");
        }
    } else {
        panic!("Expected Attributes");
    }
}

#[test]
fn test_parse_attributes_time_signature() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <attributes>
                            <time>
                                <beats>4</beats>
                                <beat-type>4</beat-type>
                            </time>
                        </attributes>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Attributes(attrs) =
        &score.parts[0].measures[0].content[0]
    {
        assert_eq!(attrs.times.len(), 1);
        if let TimeContent::Measured { signatures } = &attrs.times[0].content {
            assert_eq!(signatures.len(), 1);
            assert_eq!(signatures[0].beats, "4");
            assert_eq!(signatures[0].beat_type, "4");
        } else {
            panic!("Expected Measured time");
        }
    } else {
        panic!("Expected Attributes");
    }
}

#[test]
fn test_parse_attributes_clef() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <attributes>
                            <clef>
                                <sign>G</sign>
                                <line>2</line>
                            </clef>
                        </attributes>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Attributes(attrs) =
        &score.parts[0].measures[0].content[0]
    {
        assert_eq!(attrs.clefs.len(), 1);
        assert_eq!(attrs.clefs[0].sign, ClefSign::G);
        assert_eq!(attrs.clefs[0].line, Some(2));
    } else {
        panic!("Expected Attributes");
    }
}

#[test]
fn test_parse_attributes_complete() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <attributes>
                            <divisions>4</divisions>
                            <key>
                                <fifths>-1</fifths>
                                <mode>major</mode>
                            </key>
                            <time symbol="common">
                                <beats>4</beats>
                                <beat-type>4</beat-type>
                            </time>
                            <clef>
                                <sign>F</sign>
                                <line>4</line>
                            </clef>
                        </attributes>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Attributes(attrs) =
        &score.parts[0].measures[0].content[0]
    {
        assert_eq!(attrs.divisions, Some(4));
        assert_eq!(attrs.keys.len(), 1);
        assert_eq!(attrs.times.len(), 1);
        assert_eq!(attrs.times[0].symbol, Some(TimeSymbol::Common));
        assert_eq!(attrs.clefs.len(), 1);
        assert_eq!(attrs.clefs[0].sign, ClefSign::F);
    } else {
        panic!("Expected Attributes");
    }
}

// === Note Tests ===

#[test]
fn test_parse_note_pitched() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let NoteContent::Regular {
            full_note,
            duration,
            ..
        } = &note.content
        {
            assert_eq!(*duration, 4);
            if let PitchRestUnpitched::Pitch(p) = &full_note.content {
                assert_eq!(p.step, crate::ir::pitch::Step::C);
                assert_eq!(p.octave, 4);
            } else {
                panic!("Expected Pitch");
            }
        } else {
            panic!("Expected Regular note");
        }
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_note_with_accidental() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>F</step>
                                <alter>1</alter>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <accidental>sharp</accidental>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        assert!(note.accidental.is_some());
        assert_eq!(
            note.accidental.as_ref().unwrap().value,
            crate::ir::common::AccidentalValue::Sharp
        );
        if let NoteContent::Regular { full_note, .. } = &note.content {
            if let PitchRestUnpitched::Pitch(p) = &full_note.content {
                assert_eq!(p.alter, Some(1.0));
            } else {
                panic!("Expected Pitch");
            }
        }
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_rest() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <rest/>
                            <duration>4</duration>
                            <type>quarter</type>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let NoteContent::Regular { full_note, .. } = &note.content {
            assert!(matches!(full_note.content, PitchRestUnpitched::Rest(_)));
        } else {
            panic!("Expected Regular note");
        }
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_whole_measure_rest() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <rest measure="yes"/>
                            <duration>16</duration>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let NoteContent::Regular { full_note, .. } = &note.content {
            if let PitchRestUnpitched::Rest(r) = &full_note.content {
                assert_eq!(r.measure, Some(YesNo::Yes));
            } else {
                panic!("Expected Rest");
            }
        }
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_chord_note() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                        </note>
                        <note>
                            <chord/>
                            <pitch>
                                <step>E</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert_eq!(score.parts[0].measures[0].content.len(), 2);
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[1]
    {
        if let NoteContent::Regular { full_note, .. } = &note.content {
            assert!(full_note.chord);
        } else {
            panic!("Expected Regular note");
        }
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_grace_note() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <grace slash="yes"/>
                            <pitch>
                                <step>D</step>
                                <octave>5</octave>
                            </pitch>
                            <type>eighth</type>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let NoteContent::Grace { grace, .. } = &note.content {
            assert_eq!(grace.slash, Some(YesNo::Yes));
        } else {
            panic!("Expected Grace note");
        }
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_note_with_beam() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>1</duration>
                            <type>eighth</type>
                            <beam number="1">begin</beam>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        assert_eq!(note.beams.len(), 1);
        assert_eq!(note.beams[0].value, crate::ir::beam::BeamValue::Begin);
        assert_eq!(note.beams[0].number, 1);
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_note_with_stem() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <stem>up</stem>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        assert!(note.stem.is_some());
        assert_eq!(
            note.stem.as_ref().unwrap().value,
            crate::ir::beam::StemValue::Up
        );
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_note_with_tie() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <tie type="start"/>
                            <type>quarter</type>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let NoteContent::Regular { ties, .. } = &note.content {
            assert_eq!(ties.len(), 1);
            assert_eq!(ties[0].r#type, crate::ir::common::StartStop::Start);
        } else {
            panic!("Expected Regular note");
        }
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_note_with_dots() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>6</duration>
                            <type>quarter</type>
                            <dot/>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        assert_eq!(note.dots.len(), 1);
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_note_with_time_modification() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>2</duration>
                            <type>eighth</type>
                            <time-modification>
                                <actual-notes>3</actual-notes>
                                <normal-notes>2</normal-notes>
                            </time-modification>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        assert!(note.time_modification.is_some());
        let tm = note.time_modification.as_ref().unwrap();
        assert_eq!(tm.actual_notes, 3);
        assert_eq!(tm.normal_notes, 2);
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_note_with_voice_and_staff() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <voice>1</voice>
                            <type>quarter</type>
                            <staff>1</staff>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        assert_eq!(note.voice, Some("1".to_string()));
        assert_eq!(note.staff, Some(1));
    } else {
        panic!("Expected Note");
    }
}

// =======================================================================
// Additional tests for uncovered paths
// =======================================================================

#[test]
fn test_parse_score_unexpected_element() {
    let xml = r#"<?xml version="1.0"?>
            <unknown-root>
            </unknown-root>"#;

    let result = parse_score(xml);
    assert!(result.is_err());
    if let Err(ParseError::UnexpectedElement { element, .. }) = result {
        assert_eq!(element, "unknown-root");
    } else {
        panic!("Expected UnexpectedElement error");
    }
}

#[test]
fn test_parse_score_with_comments() {
    let xml = r#"<?xml version="1.0"?>
            <!-- This is a comment before the root -->
            <score-partwise>
                <!-- Another comment -->
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert_eq!(score.parts.len(), 1);
}

#[test]
fn test_parse_score_with_movement_number() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <movement-number>1</movement-number>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert_eq!(score.movement_number, Some("1".to_string()));
}

#[test]
fn test_parse_score_without_version() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    // Default version should be "4.0" when not specified
    assert_eq!(score.version, Some("4.0".to_string()));
}

#[test]
fn test_parse_score_with_work_element() {
    // Work element is skipped but should not cause errors
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <work>
                    <work-number>Op. 1</work-number>
                    <work-title>Sonata</work-title>
                </work>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert!(score.work.is_some());
    let work = score.work.unwrap();
    assert_eq!(work.work_title, Some("Sonata".to_string()));
}

#[test]
fn test_parse_score_with_identification_element() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <identification>
                    <creator type="composer">Bach</creator>
                </identification>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert!(score.identification.is_some());
    let id = score.identification.unwrap();
    assert_eq!(id.creators.len(), 1);
    assert_eq!(id.creators[0].value, "Bach");
    assert_eq!(id.creators[0].r#type, Some("composer".to_string()));
}

#[test]
fn test_parse_score_with_defaults_element() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <defaults>
                    <scaling>
                        <millimeters>7</millimeters>
                        <tenths>40</tenths>
                    </scaling>
                </defaults>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert!(score.defaults.is_some());
    let defaults = score.defaults.unwrap();
    assert!(defaults.scaling.is_some());
    let scaling = defaults.scaling.unwrap();
    assert_eq!(scaling.millimeters, 7.0);
    assert_eq!(scaling.tenths, 40.0);
}

#[test]
fn test_parse_score_with_credit_elements() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <credit page="1">
                    <credit-words>Title</credit-words>
                </credit>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert_eq!(score.credits.len(), 1);
    assert_eq!(score.credits[0].page, Some(1));
}

#[test]
fn test_parse_score_with_empty_defaults() {
    // Empty defaults element should be handled
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <defaults/>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert!(score.defaults.is_none());
}

#[test]
fn test_parse_score_with_unknown_element() {
    // Unknown elements should be skipped for forward compatibility
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <future-element>Some content</future-element>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert_eq!(score.parts.len(), 1);
}

#[test]
fn test_parse_empty_measure() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert_eq!(score.parts[0].measures[0].number, "1");
    assert!(score.parts[0].measures[0].content.is_empty());
}

#[test]
fn test_parse_measure_with_non_controlling() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1" non-controlling="yes">
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert_eq!(score.parts[0].measures[0].non_controlling, Some(YesNo::Yes));
}

#[test]
fn test_parse_part_group_with_barline() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <part-group type="start" number="1">
                        <group-name>Strings</group-name>
                        <group-barline>yes</group-barline>
                    </part-group>
                    <score-part id="P1">
                        <part-name>Violin</part-name>
                    </score-part>
                    <part-group type="stop" number="1"/>
                </part-list>
                <part id="P1"><measure number="1"/></part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let PartListElement::PartGroup(pg) = &score.part_list.content[0] {
        assert!(pg.group_barline.is_some());
    } else {
        panic!("Expected PartGroup");
    }
}

#[test]
fn test_parse_part_group_mensurstrich() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <part-group type="start" number="1">
                        <group-barline>Mensurstrich</group-barline>
                    </part-group>
                    <score-part id="P1">
                        <part-name>Violin</part-name>
                    </score-part>
                    <part-group type="stop" number="1"/>
                </part-list>
                <part id="P1"><measure number="1"/></part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let PartListElement::PartGroup(pg) = &score.part_list.content[0] {
        use crate::ir::part::GroupBarlineValue;
        assert_eq!(
            pg.group_barline.as_ref().unwrap().value,
            GroupBarlineValue::Mensurstrich
        );
    } else {
        panic!("Expected PartGroup");
    }
}

#[test]
fn test_parse_part_group_symbol_values() {
    let symbols = ["none", "brace", "line", "bracket", "square"];
    for symbol in symbols {
        let xml = format!(
            r#"<?xml version="1.0"?>
                <score-partwise>
                    <part-list>
                        <part-group type="start" number="1">
                            <group-symbol>{}</group-symbol>
                        </part-group>
                        <score-part id="P1">
                            <part-name>Test</part-name>
                        </score-part>
                        <part-group type="stop" number="1"/>
                    </part-list>
                    <part id="P1"><measure number="1"/></part>
                </score-partwise>"#,
            symbol
        );

        let score = parse_score(&xml).unwrap();
        if let PartListElement::PartGroup(pg) = &score.part_list.content[0] {
            assert!(pg.group_symbol.is_some(), "Failed for symbol: {}", symbol);
        } else {
            panic!("Expected PartGroup");
        }
    }
}

#[test]
fn test_parse_part_group_invalid_symbol() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <part-group type="start" number="1">
                        <group-symbol>invalid</group-symbol>
                    </part-group>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1"><measure number="1"/></part>
            </score-partwise>"#;

    let result = parse_score(xml);
    assert!(result.is_err());
}

#[test]
fn test_parse_part_group_invalid_barline() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <part-group type="start" number="1">
                        <group-barline>invalid</group-barline>
                    </part-group>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1"><measure number="1"/></part>
            </score-partwise>"#;

    let result = parse_score(xml);
    assert!(result.is_err());
}

#[test]
fn test_parse_part_group_empty_symbol() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <part-group type="start" number="1">
                        <group-symbol/>
                    </part-group>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                    <part-group type="stop" number="1"/>
                </part-list>
                <part id="P1"><measure number="1"/></part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let PartListElement::PartGroup(pg) = &score.part_list.content[0] {
        use crate::ir::attributes::GroupSymbolValue;
        assert_eq!(
            pg.group_symbol.as_ref().unwrap().value,
            GroupSymbolValue::None
        );
    } else {
        panic!("Expected PartGroup");
    }
}

#[test]
fn test_parse_part_group_with_group_time() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <part-group type="start" number="1">
                        <group-time/>
                    </part-group>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                    <part-group type="stop" number="1"/>
                </part-list>
                <part id="P1"><measure number="1"/></part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let PartListElement::PartGroup(pg) = &score.part_list.content[0] {
        assert!(pg.group_time.is_some());
    } else {
        panic!("Expected PartGroup");
    }
}

#[test]
fn test_parse_score_part_with_empty_part_name() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name/>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let PartListElement::ScorePart(sp) = &score.part_list.content[0] {
        assert_eq!(sp.part_name.value, "");
    } else {
        panic!("Expected ScorePart");
    }
}

#[test]
fn test_parse_score_part_with_group() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Violin</part-name>
                        <group>Strings</group>
                        <group>Orchestra</group>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let PartListElement::ScorePart(sp) = &score.part_list.content[0] {
        assert_eq!(sp.group.len(), 2);
        assert_eq!(sp.group[0], "Strings");
        assert_eq!(sp.group[1], "Orchestra");
    } else {
        panic!("Expected ScorePart");
    }
}

#[test]
fn test_parse_unpitched_note() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Drums</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <unpitched>
                                <display-step>E</display-step>
                                <display-octave>4</display-octave>
                            </unpitched>
                            <duration>4</duration>
                            <type>quarter</type>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let NoteContent::Regular { full_note, .. } = &note.content {
            if let PitchRestUnpitched::Unpitched(u) = &full_note.content {
                assert_eq!(u.display_step, Some(crate::ir::pitch::Step::E));
                assert_eq!(u.display_octave, Some(4));
            } else {
                panic!("Expected Unpitched");
            }
        } else {
            panic!("Expected Regular note");
        }
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_empty_unpitched_note() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Drums</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <unpitched/>
                            <duration>4</duration>
                            <type>quarter</type>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let NoteContent::Regular { full_note, .. } = &note.content {
            if let PitchRestUnpitched::Unpitched(u) = &full_note.content {
                assert!(u.display_step.is_none());
                assert!(u.display_octave.is_none());
            } else {
                panic!("Expected Unpitched");
            }
        }
    }
}

#[test]
fn test_parse_rest_with_display_position() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <rest>
                                <display-step>B</display-step>
                                <display-octave>4</display-octave>
                            </rest>
                            <duration>4</duration>
                            <type>quarter</type>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let NoteContent::Regular { full_note, .. } = &note.content {
            if let PitchRestUnpitched::Rest(r) = &full_note.content {
                assert_eq!(r.display_step, Some(crate::ir::pitch::Step::B));
                assert_eq!(r.display_octave, Some(4));
            } else {
                panic!("Expected Rest");
            }
        }
    }
}

#[test]
fn test_parse_note_with_notehead() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notehead>diamond</notehead>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        assert!(note.notehead.is_some());
        use crate::ir::beam::NoteheadValue;
        assert_eq!(
            note.notehead.as_ref().unwrap().value,
            NoteheadValue::Diamond
        );
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_cue_note() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <cue/>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let NoteContent::Cue {
            full_note,
            duration,
        } = &note.content
        {
            assert_eq!(*duration, 4);
            if let PitchRestUnpitched::Pitch(p) = &full_note.content {
                assert_eq!(p.step, crate::ir::pitch::Step::C);
            } else {
                panic!("Expected Pitch");
            }
        } else {
            panic!("Expected Cue note");
        }
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_key_with_cancel() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <attributes>
                            <key>
                                <cancel>-2</cancel>
                                <fifths>1</fifths>
                                <mode>major</mode>
                            </key>
                        </attributes>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Attributes(attrs) =
        &score.parts[0].measures[0].content[0]
    {
        if let KeyContent::Traditional(tk) = &attrs.keys[0].content {
            assert!(tk.cancel.is_some());
            assert_eq!(tk.cancel.as_ref().unwrap().fifths, -2);
        } else {
            panic!("Expected Traditional key");
        }
    } else {
        panic!("Expected Attributes");
    }
}

#[test]
fn test_parse_time_senza_misura() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <attributes>
                            <time>
                                <senza-misura/>
                            </time>
                        </attributes>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Attributes(attrs) =
        &score.parts[0].measures[0].content[0]
    {
        if let TimeContent::SenzaMisura(_) = &attrs.times[0].content {
            // Success
        } else {
            panic!("Expected SenzaMisura time");
        }
    } else {
        panic!("Expected Attributes");
    }
}

#[test]
fn test_parse_clef_octave_change() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <attributes>
                            <clef>
                                <sign>G</sign>
                                <line>2</line>
                                <clef-octave-change>-1</clef-octave-change>
                            </clef>
                        </attributes>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Attributes(attrs) =
        &score.parts[0].measures[0].content[0]
    {
        assert_eq!(attrs.clefs[0].octave_change, Some(-1));
    } else {
        panic!("Expected Attributes");
    }
}

#[test]
fn test_parse_attributes_staves() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Piano</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <attributes>
                            <divisions>4</divisions>
                            <staves>2</staves>
                        </attributes>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Attributes(attrs) =
        &score.parts[0].measures[0].content[0]
    {
        assert_eq!(attrs.staves, Some(2));
    } else {
        panic!("Expected Attributes");
    }
}

#[test]
fn test_parse_forward_without_optional_elements() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <forward>
                            <duration>4</duration>
                        </forward>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Forward(forward) =
        &score.parts[0].measures[0].content[0]
    {
        assert_eq!(forward.duration, 4);
        assert!(forward.voice.is_none());
        assert!(forward.staff.is_none());
    } else {
        panic!("Expected Forward");
    }
}

#[test]
fn test_parse_all_clef_signs() {
    let signs = ["G", "F", "C", "percussion", "TAB", "jianpu", "none"];
    for sign in signs {
        let xml = format!(
            r#"<?xml version="1.0"?>
                <score-partwise>
                    <part-list>
                        <score-part id="P1">
                            <part-name>Test</part-name>
                        </score-part>
                    </part-list>
                    <part id="P1">
                        <measure number="1">
                            <attributes>
                                <clef>
                                    <sign>{}</sign>
                                </clef>
                            </attributes>
                        </measure>
                    </part>
                </score-partwise>"#,
            sign
        );

        let score = parse_score(&xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Attributes(attrs) =
            &score.parts[0].measures[0].content[0]
        {
            assert!(!attrs.clefs.is_empty(), "Failed for sign: {}", sign);
        } else {
            panic!("Expected Attributes for sign: {}", sign);
        }
    }
}

#[test]
fn test_parse_all_mode_values() {
    let modes = [
        "major",
        "minor",
        "dorian",
        "phrygian",
        "lydian",
        "mixolydian",
        "aeolian",
        "ionian",
        "locrian",
    ];
    for mode in modes {
        let xml = format!(
            r#"<?xml version="1.0"?>
                <score-partwise>
                    <part-list>
                        <score-part id="P1">
                            <part-name>Test</part-name>
                        </score-part>
                    </part-list>
                    <part id="P1">
                        <measure number="1">
                            <attributes>
                                <key>
                                    <fifths>0</fifths>
                                    <mode>{}</mode>
                                </key>
                            </attributes>
                        </measure>
                    </part>
                </score-partwise>"#,
            mode
        );

        let score = parse_score(&xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Attributes(attrs) =
            &score.parts[0].measures[0].content[0]
        {
            if let KeyContent::Traditional(tk) = &attrs.keys[0].content {
                assert!(tk.mode.is_some(), "Failed for mode: {}", mode);
            } else {
                panic!("Expected Traditional key for mode: {}", mode);
            }
        } else {
            panic!("Expected Attributes for mode: {}", mode);
        }
    }
}

#[test]
fn test_parse_note_without_type() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        assert!(note.r#type.is_none());
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_grace_note_with_steal_time() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <grace steal-time-previous="50" steal-time-following="25"/>
                            <pitch>
                                <step>D</step>
                                <octave>4</octave>
                            </pitch>
                            <type>eighth</type>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let NoteContent::Grace { grace, .. } = &note.content {
            assert_eq!(grace.steal_time_previous, Some(50.0));
            assert_eq!(grace.steal_time_following, Some(25.0));
        } else {
            panic!("Expected Grace note");
        }
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_double_dotted_note() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>7</duration>
                            <type>quarter</type>
                            <dot/>
                            <dot/>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        assert_eq!(note.dots.len(), 2);
    } else {
        panic!("Expected Note");
    }
}

// =======================================================================
// Multi-Voice Tests (Task 3.3)
// =======================================================================

#[test]
fn test_parse_two_voice_measure_with_backup() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <attributes>
                            <divisions>4</divisions>
                        </attributes>
                        <note>
                            <pitch>
                                <step>E</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>16</duration>
                            <voice>1</voice>
                            <type>whole</type>
                        </note>
                        <backup>
                            <duration>16</duration>
                        </backup>
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>3</octave>
                            </pitch>
                            <duration>16</duration>
                            <voice>2</voice>
                            <type>whole</type>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    let content = &score.parts[0].measures[0].content;
    assert_eq!(content.len(), 4); // attributes, note, backup, note

    // Check first note is voice 1
    if let crate::ir::measure::MusicDataElement::Note(note) = &content[1] {
        assert_eq!(note.voice, Some("1".to_string()));
    } else {
        panic!("Expected Note at index 1");
    }

    // Check backup element
    if let crate::ir::measure::MusicDataElement::Backup(backup) = &content[2] {
        assert_eq!(backup.duration, 16);
    } else {
        panic!("Expected Backup at index 2");
    }

    // Check second note is voice 2
    if let crate::ir::measure::MusicDataElement::Note(note) = &content[3] {
        assert_eq!(note.voice, Some("2".to_string()));
    } else {
        panic!("Expected Note at index 3");
    }
}

#[test]
fn test_parse_forward_element_with_voice_and_staff() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <forward>
                            <duration>8</duration>
                            <voice>2</voice>
                            <staff>1</staff>
                        </forward>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Forward(forward) =
        &score.parts[0].measures[0].content[0]
    {
        assert_eq!(forward.duration, 8);
        assert_eq!(forward.voice, Some("2".to_string()));
        assert_eq!(forward.staff, Some(1));
    } else {
        panic!("Expected Forward");
    }
}

#[test]
fn test_parse_voice_assignment_preserved() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <voice>1</voice>
                            <type>quarter</type>
                        </note>
                        <note>
                            <pitch>
                                <step>D</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <voice>1</voice>
                            <type>quarter</type>
                        </note>
                        <backup>
                            <duration>8</duration>
                        </backup>
                        <note>
                            <pitch>
                                <step>G</step>
                                <octave>3</octave>
                            </pitch>
                            <duration>8</duration>
                            <voice>2</voice>
                            <type>half</type>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    let content = &score.parts[0].measures[0].content;

    // Verify voice assignments are preserved
    let mut voice_1_count = 0;
    let mut voice_2_count = 0;

    for element in content {
        if let crate::ir::measure::MusicDataElement::Note(note) = element {
            match note.voice.as_deref() {
                Some("1") => voice_1_count += 1,
                Some("2") => voice_2_count += 1,
                _ => {}
            }
        }
    }

    assert_eq!(voice_1_count, 2, "Expected 2 notes in voice 1");
    assert_eq!(voice_2_count, 1, "Expected 1 note in voice 2");
}

// =======================================================================
// Barline Tests (Task 3.4)
// =======================================================================

#[test]
fn test_parse_barline_simple_forward_repeat() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline location="left">
                            <bar-style>heavy-light</bar-style>
                            <repeat direction="forward"/>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Barline(barline) =
        &score.parts[0].measures[0].content[0]
    {
        assert_eq!(
            barline.location,
            Some(crate::ir::common::RightLeftMiddle::Left)
        );
        assert_eq!(
            barline.bar_style,
            Some(crate::ir::attributes::BarStyle::HeavyLight)
        );
        assert!(barline.repeat.is_some());
        let repeat = barline.repeat.as_ref().unwrap();
        assert_eq!(
            repeat.direction,
            crate::ir::common::BackwardForward::Forward
        );
    } else {
        panic!("Expected Barline");
    }
}

#[test]
fn test_parse_barline_backward_repeat() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline location="right">
                            <bar-style>light-heavy</bar-style>
                            <repeat direction="backward" times="2"/>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Barline(barline) =
        &score.parts[0].measures[0].content[0]
    {
        assert_eq!(
            barline.location,
            Some(crate::ir::common::RightLeftMiddle::Right)
        );
        assert_eq!(
            barline.bar_style,
            Some(crate::ir::attributes::BarStyle::LightHeavy)
        );
        let repeat = barline.repeat.as_ref().unwrap();
        assert_eq!(
            repeat.direction,
            crate::ir::common::BackwardForward::Backward
        );
        assert_eq!(repeat.times, Some(2));
    } else {
        panic!("Expected Barline");
    }
}

#[test]
fn test_parse_barline_volta_first_ending() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline location="left">
                            <ending number="1" type="start">1.</ending>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Barline(barline) =
        &score.parts[0].measures[0].content[0]
    {
        assert!(barline.ending.is_some());
        let ending = barline.ending.as_ref().unwrap();
        assert_eq!(
            ending.r#type,
            crate::ir::common::StartStopDiscontinue::Start
        );
        assert_eq!(ending.number, "1");
        assert_eq!(ending.text, Some("1.".to_string()));
    } else {
        panic!("Expected Barline");
    }
}

#[test]
fn test_parse_barline_volta_second_ending() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline location="left">
                            <ending number="2" type="start">2.</ending>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Barline(barline) =
        &score.parts[0].measures[0].content[0]
    {
        let ending = barline.ending.as_ref().unwrap();
        assert_eq!(ending.number, "2");
        assert_eq!(ending.text, Some("2.".to_string()));
    } else {
        panic!("Expected Barline");
    }
}

#[test]
fn test_parse_barline_ending_stop() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline location="right">
                            <ending number="1" type="stop"/>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Barline(barline) =
        &score.parts[0].measures[0].content[0]
    {
        let ending = barline.ending.as_ref().unwrap();
        assert_eq!(ending.r#type, crate::ir::common::StartStopDiscontinue::Stop);
    } else {
        panic!("Expected Barline");
    }
}

#[test]
fn test_parse_barline_ending_discontinue() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline location="right">
                            <ending number="1" type="discontinue"/>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Barline(barline) =
        &score.parts[0].measures[0].content[0]
    {
        let ending = barline.ending.as_ref().unwrap();
        assert_eq!(
            ending.r#type,
            crate::ir::common::StartStopDiscontinue::Discontinue
        );
    } else {
        panic!("Expected Barline");
    }
}

#[test]
fn test_parse_barline_with_segno() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline location="left">
                            <segno/>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Barline(barline) =
        &score.parts[0].measures[0].content[0]
    {
        assert!(barline.segno.is_some());
    } else {
        panic!("Expected Barline");
    }
}

#[test]
fn test_parse_barline_with_coda() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline location="left">
                            <coda/>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Barline(barline) =
        &score.parts[0].measures[0].content[0]
    {
        assert!(barline.coda.is_some());
    } else {
        panic!("Expected Barline");
    }
}

#[test]
fn test_parse_barline_with_fermata() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline location="right">
                            <fermata type="upright"/>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Barline(barline) =
        &score.parts[0].measures[0].content[0]
    {
        assert_eq!(barline.fermatas.len(), 1);
        assert_eq!(
            barline.fermatas[0].r#type,
            Some(crate::ir::common::UprightInverted::Upright)
        );
    } else {
        panic!("Expected Barline");
    }
}

#[test]
fn test_parse_barline_all_bar_styles() {
    let styles = [
        ("regular", crate::ir::attributes::BarStyle::Regular),
        ("dotted", crate::ir::attributes::BarStyle::Dotted),
        ("dashed", crate::ir::attributes::BarStyle::Dashed),
        ("heavy", crate::ir::attributes::BarStyle::Heavy),
        ("light-light", crate::ir::attributes::BarStyle::LightLight),
        ("light-heavy", crate::ir::attributes::BarStyle::LightHeavy),
        ("heavy-light", crate::ir::attributes::BarStyle::HeavyLight),
        ("heavy-heavy", crate::ir::attributes::BarStyle::HeavyHeavy),
        ("tick", crate::ir::attributes::BarStyle::Tick),
        ("short", crate::ir::attributes::BarStyle::Short),
        ("none", crate::ir::attributes::BarStyle::None),
    ];

    for (style_str, expected_style) in styles {
        let xml = format!(
            r#"<?xml version="1.0"?>
                <score-partwise>
                    <part-list>
                        <score-part id="P1">
                            <part-name>Test</part-name>
                        </score-part>
                    </part-list>
                    <part id="P1">
                        <measure number="1">
                            <barline>
                                <bar-style>{}</bar-style>
                            </barline>
                        </measure>
                    </part>
                </score-partwise>"#,
            style_str
        );

        let score = parse_score(&xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Barline(barline) =
            &score.parts[0].measures[0].content[0]
        {
            assert_eq!(
                barline.bar_style,
                Some(expected_style),
                "Failed for style: {}",
                style_str
            );
        } else {
            panic!("Expected Barline for style: {}", style_str);
        }
    }
}

#[test]
fn test_parse_barline_repeat_with_winged() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline location="right">
                            <repeat direction="backward" winged="curved"/>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Barline(barline) =
        &score.parts[0].measures[0].content[0]
    {
        let repeat = barline.repeat.as_ref().unwrap();
        assert_eq!(repeat.winged, Some(crate::ir::attributes::Winged::Curved));
    } else {
        panic!("Expected Barline");
    }
}

#[test]
fn test_parse_barline_location_middle() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline location="middle">
                            <bar-style>dashed</bar-style>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Barline(barline) =
        &score.parts[0].measures[0].content[0]
    {
        assert_eq!(
            barline.location,
            Some(crate::ir::common::RightLeftMiddle::Middle)
        );
    } else {
        panic!("Expected Barline");
    }
}

#[test]
fn test_parse_barline_with_wavy_line() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline>
                            <wavy-line type="start" number="1"/>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Barline(barline) =
        &score.parts[0].measures[0].content[0]
    {
        assert!(barline.wavy_line.is_some());
        let wavy = barline.wavy_line.as_ref().unwrap();
        assert_eq!(wavy.r#type, crate::ir::common::StartStopContinue::Start);
        assert_eq!(wavy.number, Some(1));
    } else {
        panic!("Expected Barline");
    }
}

#[test]
fn test_parse_barline_empty_repeat() {
    // Test parsing repeat as an empty element
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline>
                            <repeat direction="forward"/>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Barline(barline) =
        &score.parts[0].measures[0].content[0]
    {
        assert!(barline.repeat.is_some());
    } else {
        panic!("Expected Barline");
    }
}

#[test]
fn test_parse_barline_ending_with_attributes() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline>
                            <ending number="1, 2" type="start" end-length="30" text-x="5" text-y="-10">1, 2.</ending>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Barline(barline) =
        &score.parts[0].measures[0].content[0]
    {
        let ending = barline.ending.as_ref().unwrap();
        assert_eq!(ending.number, "1, 2");
        assert_eq!(ending.text, Some("1, 2.".to_string()));
        assert_eq!(ending.end_length, Some(30.0));
        assert_eq!(ending.text_x, Some(5.0));
        assert_eq!(ending.text_y, Some(-10.0));
    } else {
        panic!("Expected Barline");
    }
}

// =======================================================================
// Direction Tests (Milestone 4, Task 4.1-4.3)
// =======================================================================

#[test]
fn test_parse_direction_with_dynamics_f() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <direction placement="below">
                            <direction-type>
                                <dynamics><f/></dynamics>
                            </direction-type>
                        </direction>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Direction(dir) =
        &score.parts[0].measures[0].content[0]
    {
        assert_eq!(dir.placement, Some(crate::ir::common::AboveBelow::Below));
        assert_eq!(dir.direction_types.len(), 1);
        if let crate::ir::direction::DirectionTypeContent::Dynamics(d) =
            &dir.direction_types[0].content
        {
            assert_eq!(d.content.len(), 1);
            assert_eq!(d.content[0], crate::ir::direction::DynamicElement::F);
        } else {
            panic!("Expected Dynamics content");
        }
    } else {
        panic!("Expected Direction");
    }
}

#[test]
fn test_parse_direction_with_wedge_crescendo() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <direction>
                            <direction-type>
                                <wedge type="crescendo"/>
                            </direction-type>
                        </direction>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Direction(dir) =
        &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::direction::DirectionTypeContent::Wedge(w) =
            &dir.direction_types[0].content
        {
            assert_eq!(w.r#type, crate::ir::direction::WedgeType::Crescendo);
        } else {
            panic!("Expected Wedge content");
        }
    } else {
        panic!("Expected Direction");
    }
}

#[test]
fn test_parse_direction_with_metronome() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <direction>
                            <direction-type>
                                <metronome>
                                    <beat-unit>quarter</beat-unit>
                                    <per-minute>120</per-minute>
                                </metronome>
                            </direction-type>
                            <sound tempo="120"/>
                        </direction>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Direction(dir) =
        &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::direction::DirectionTypeContent::Metronome(m) =
            &dir.direction_types[0].content
        {
            if let crate::ir::direction::MetronomeContent::PerMinute { per_minute, .. } = &m.content
            {
                assert_eq!(per_minute.value, "120");
            } else {
                panic!("Expected PerMinute content");
            }
        } else {
            panic!("Expected Metronome content");
        }
        // Check sound element
        assert!(dir.sound.is_some());
        assert_eq!(dir.sound.as_ref().unwrap().tempo, Some(120.0));
    } else {
        panic!("Expected Direction");
    }
}

#[test]
fn test_parse_direction_with_words() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <direction>
                            <direction-type>
                                <words>cresc.</words>
                            </direction-type>
                        </direction>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Direction(dir) =
        &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::direction::DirectionTypeContent::Words(w) =
            &dir.direction_types[0].content
        {
            assert_eq!(w.len(), 1);
            assert_eq!(w[0].value, "cresc.");
        } else {
            panic!("Expected Words content");
        }
    } else {
        panic!("Expected Direction");
    }
}

#[test]
fn test_parse_direction_with_pedal() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <direction>
                            <direction-type>
                                <pedal type="start" line="yes"/>
                            </direction-type>
                        </direction>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Direction(dir) =
        &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::direction::DirectionTypeContent::Pedal(p) =
            &dir.direction_types[0].content
        {
            assert_eq!(p.r#type, crate::ir::direction::PedalType::Start);
            assert_eq!(p.line, Some(YesNo::Yes));
        } else {
            panic!("Expected Pedal content");
        }
    } else {
        panic!("Expected Direction");
    }
}

// =======================================================================
// Notations Tests (Milestone 4, Task 4.4-4.5)
// =======================================================================

#[test]
fn test_parse_note_with_tied_notation() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <tied type="start"/>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        assert_eq!(note.notations.len(), 1);
        assert_eq!(note.notations[0].content.len(), 1);
        if let crate::ir::notation::NotationContent::Tied(t) = &note.notations[0].content[0] {
            assert_eq!(t.r#type, crate::ir::common::StartStopContinue::Start);
        } else {
            panic!("Expected Tied notation");
        }
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_note_with_slur() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <slur type="start" number="1"/>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Slur(s) = &note.notations[0].content[0] {
            assert_eq!(s.r#type, crate::ir::common::StartStopContinue::Start);
            assert_eq!(s.number, 1);
        } else {
            panic!("Expected Slur notation");
        }
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_note_with_articulations() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <articulations>
                                    <staccato/>
                                    <accent/>
                                </articulations>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Articulations(a) =
            &note.notations[0].content[0]
        {
            assert_eq!(a.content.len(), 2);
        } else {
            panic!("Expected Articulations notation");
        }
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_note_with_ornaments_trill() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <ornaments>
                                    <trill-mark/>
                                </ornaments>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Ornaments(o) = &note.notations[0].content[0] {
            assert_eq!(o.content.len(), 1);
            if let crate::ir::notation::OrnamentElement::TrillMark(_) = &o.content[0].ornament {
                // Success
            } else {
                panic!("Expected TrillMark ornament");
            }
        } else {
            panic!("Expected Ornaments notation");
        }
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_note_with_fermata() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <fermata type="upright"/>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Fermata(f) = &note.notations[0].content[0] {
            assert_eq!(f.r#type, Some(crate::ir::common::UprightInverted::Upright));
        } else {
            panic!("Expected Fermata notation");
        }
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_note_with_tuplet() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>2</duration>
                            <type>eighth</type>
                            <time-modification>
                                <actual-notes>3</actual-notes>
                                <normal-notes>2</normal-notes>
                            </time-modification>
                            <notations>
                                <tuplet type="start" bracket="yes"/>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Tuplet(t) = &note.notations[0].content[0] {
            assert_eq!(t.r#type, crate::ir::common::StartStop::Start);
            assert_eq!(t.bracket, Some(YesNo::Yes));
        } else {
            panic!("Expected Tuplet notation");
        }
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_note_with_technical_fingering() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <technical>
                                    <fingering>1</fingering>
                                </technical>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Technical(t) = &note.notations[0].content[0] {
            assert_eq!(t.content.len(), 1);
            if let crate::ir::notation::TechnicalElement::Fingering(f) = &t.content[0] {
                assert_eq!(f.value, "1");
            } else {
                panic!("Expected Fingering technical");
            }
        } else {
            panic!("Expected Technical notation");
        }
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_note_with_arpeggiate() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <arpeggiate direction="up"/>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Arpeggiate(a) = &note.notations[0].content[0] {
            assert_eq!(a.direction, Some(crate::ir::common::UpDown::Up));
        } else {
            panic!("Expected Arpeggiate notation");
        }
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_all_dynamics() {
    let dynamics = [
        "p", "pp", "ppp", "pppp", "ppppp", "pppppp", "f", "ff", "fff", "ffff", "fffff", "ffffff",
        "mp", "mf", "sf", "sfp", "sfpp", "fp", "rf", "rfz", "sfz", "sffz", "fz", "n", "pf", "sfzp",
    ];
    for d in dynamics {
        let xml = format!(
            r#"<?xml version="1.0"?>
                <score-partwise>
                    <part-list>
                        <score-part id="P1">
                            <part-name>Test</part-name>
                        </score-part>
                    </part-list>
                    <part id="P1">
                        <measure number="1">
                            <direction>
                                <direction-type>
                                    <dynamics><{}/></dynamics>
                                </direction-type>
                            </direction>
                        </measure>
                    </part>
                </score-partwise>"#,
            d
        );

        let result = parse_score(&xml);
        assert!(
            result.is_ok(),
            "Failed to parse dynamics: {} - {:?}",
            d,
            result.err()
        );
    }
}

// =======================================================================
// Lyric Parsing Tests (Milestone 5, Task 5.1)
// =======================================================================

#[test]
fn test_parse_note_with_simple_lyric() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <lyric number="1">
                                <syllabic>single</syllabic>
                                <text>love</text>
                            </lyric>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        assert_eq!(note.lyrics.len(), 1);
        assert_eq!(note.lyrics[0].number, Some("1".to_string()));
        if let crate::ir::lyric::LyricContent::Syllable { syllabic, text, .. } =
            &note.lyrics[0].content
        {
            assert_eq!(*syllabic, Some(crate::ir::lyric::Syllabic::Single));
            assert_eq!(text.value, "love");
        } else {
            panic!("Expected Syllable content");
        }
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_note_with_multi_verse_lyrics() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <lyric number="1">
                                <syllabic>begin</syllabic>
                                <text>Hap</text>
                            </lyric>
                            <lyric number="2">
                                <syllabic>single</syllabic>
                                <text>Joy</text>
                            </lyric>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        assert_eq!(note.lyrics.len(), 2);
        assert_eq!(note.lyrics[0].number, Some("1".to_string()));
        assert_eq!(note.lyrics[1].number, Some("2".to_string()));

        if let crate::ir::lyric::LyricContent::Syllable { syllabic, text, .. } =
            &note.lyrics[0].content
        {
            assert_eq!(*syllabic, Some(crate::ir::lyric::Syllabic::Begin));
            assert_eq!(text.value, "Hap");
        }

        if let crate::ir::lyric::LyricContent::Syllable { syllabic, text, .. } =
            &note.lyrics[1].content
        {
            assert_eq!(*syllabic, Some(crate::ir::lyric::Syllabic::Single));
            assert_eq!(text.value, "Joy");
        }
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_lyric_with_extend() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <lyric number="1">
                                <syllabic>end</syllabic>
                                <text>day</text>
                                <extend type="start"/>
                            </lyric>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::lyric::LyricContent::Syllable { extend, .. } = &note.lyrics[0].content {
            assert!(extend.is_some());
            assert_eq!(
                extend.as_ref().unwrap().r#type,
                Some(crate::ir::common::StartStopContinue::Start)
            );
        } else {
            panic!("Expected Syllable content");
        }
    } else {
        panic!("Expected Note");
    }
}

#[test]
fn test_parse_lyric_laughing_and_humming() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <lyric number="1">
                                <laughing/>
                            </lyric>
                        </note>
                        <note>
                            <pitch>
                                <step>D</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <lyric number="1">
                                <humming/>
                            </lyric>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        assert_eq!(
            note.lyrics[0].content,
            crate::ir::lyric::LyricContent::Laughing
        );
    }
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[1]
    {
        assert_eq!(
            note.lyrics[0].content,
            crate::ir::lyric::LyricContent::Humming
        );
    }
}

// =======================================================================
// Score Header Parsing Tests (Milestone 5, Task 5.4)
// =======================================================================

#[test]
fn test_parse_work_element() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <work>
                    <work-number>Op. 27, No. 2</work-number>
                    <work-title>Piano Sonata No. 14</work-title>
                </work>
                <movement-number>1</movement-number>
                <movement-title>Adagio sostenuto</movement-title>
                <part-list>
                    <score-part id="P1">
                        <part-name>Piano</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert!(score.work.is_some());
    let work = score.work.as_ref().unwrap();
    assert_eq!(work.work_number, Some("Op. 27, No. 2".to_string()));
    assert_eq!(work.work_title, Some("Piano Sonata No. 14".to_string()));
    assert_eq!(score.movement_number, Some("1".to_string()));
    assert_eq!(score.movement_title, Some("Adagio sostenuto".to_string()));
}

#[test]
fn test_parse_identification_element() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <identification>
                    <creator type="composer">Ludwig van Beethoven</creator>
                    <creator type="lyricist">Unknown</creator>
                    <rights>Copyright 2024</rights>
                    <encoding>
                        <software>Fermata</software>
                        <encoding-date>2024-01-01</encoding-date>
                    </encoding>
                    <source>Manuscript</source>
                </identification>
                <part-list>
                    <score-part id="P1">
                        <part-name>Piano</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert!(score.identification.is_some());
    let id = score.identification.as_ref().unwrap();
    assert_eq!(id.creators.len(), 2);
    assert_eq!(id.creators[0].r#type, Some("composer".to_string()));
    assert_eq!(id.creators[0].value, "Ludwig van Beethoven");
    assert_eq!(id.rights.len(), 1);
    assert_eq!(id.rights[0].value, "Copyright 2024");
    assert!(id.encoding.is_some());
    assert_eq!(id.source, Some("Manuscript".to_string()));
}

#[test]
fn test_parse_defaults_with_scaling() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <defaults>
                    <scaling>
                        <millimeters>7.056</millimeters>
                        <tenths>40</tenths>
                    </scaling>
                    <page-layout>
                        <page-height>1683</page-height>
                        <page-width>1190</page-width>
                        <page-margins type="both">
                            <left-margin>70</left-margin>
                            <right-margin>70</right-margin>
                            <top-margin>88</top-margin>
                            <bottom-margin>88</bottom-margin>
                        </page-margins>
                    </page-layout>
                </defaults>
                <part-list>
                    <score-part id="P1">
                        <part-name>Piano</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert!(score.defaults.is_some());
    let defaults = score.defaults.as_ref().unwrap();
    assert!(defaults.scaling.is_some());
    let scaling = defaults.scaling.as_ref().unwrap();
    assert_eq!(scaling.millimeters, 7.056);
    assert_eq!(scaling.tenths, 40.0);
    assert!(defaults.page_layout.is_some());
    let page_layout = defaults.page_layout.as_ref().unwrap();
    assert_eq!(page_layout.page_height, Some(1683.0));
    assert_eq!(page_layout.page_width, Some(1190.0));
    assert_eq!(page_layout.page_margins.len(), 1);
}

#[test]
fn test_parse_credit_element() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <credit page="1">
                    <credit-type>title</credit-type>
                    <credit-words justify="center" halign="center" valign="top">Symphony No. 5</credit-words>
                </credit>
                <credit page="1">
                    <credit-type>composer</credit-type>
                    <credit-words>Ludwig van Beethoven</credit-words>
                </credit>
                <part-list>
                    <score-part id="P1">
                        <part-name>Piano</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert_eq!(score.credits.len(), 2);
    assert_eq!(score.credits[0].page, Some(1));
    assert_eq!(score.credits[0].content.len(), 2);
    if let crate::ir::score::CreditContent::CreditType(ct) = &score.credits[0].content[0] {
        assert_eq!(ct, "title");
    }
    if let crate::ir::score::CreditContent::CreditWords(cw) = &score.credits[0].content[1] {
        assert_eq!(cw.value, "Symphony No. 5");
    }
}

#[test]
fn test_parse_encoding_with_supports() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <identification>
                    <encoding>
                        <software>Fermata 1.0</software>
                        <encoding-date>2024-01-15</encoding-date>
                        <supports element="accidental" type="yes"/>
                        <supports element="beam" type="yes"/>
                        <supports element="stem" type="yes"/>
                    </encoding>
                </identification>
                <part-list>
                    <score-part id="P1">
                        <part-name>Piano</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert!(score.identification.is_some());
    let encoding = score
        .identification
        .as_ref()
        .unwrap()
        .encoding
        .as_ref()
        .unwrap();
    assert!(encoding.content.len() >= 5);

    // Check for supports elements
    let mut supports_count = 0;
    for item in &encoding.content {
        if let crate::ir::common::EncodingContent::Supports(s) = item {
            supports_count += 1;
            assert_eq!(s.r#type, YesNo::Yes);
        }
    }
    assert_eq!(supports_count, 3);
}

// =======================================================================
// Complex Tuplet Tests (Milestone 5, Task 5.5)
// =======================================================================

#[test]
fn test_parse_tuplet_with_time_modification() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>2</duration>
                            <type>eighth</type>
                            <time-modification>
                                <actual-notes>3</actual-notes>
                                <normal-notes>2</normal-notes>
                                <normal-type>eighth</normal-type>
                            </time-modification>
                            <notations>
                                <tuplet type="start" number="1" bracket="yes" show-number="actual"/>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        // Check time modification
        assert!(note.time_modification.is_some());
        let tm = note.time_modification.as_ref().unwrap();
        assert_eq!(tm.actual_notes, 3);
        assert_eq!(tm.normal_notes, 2);

        // Check tuplet notation
        assert!(!note.notations.is_empty());
        if let crate::ir::notation::NotationContent::Tuplet(t) = &note.notations[0].content[0] {
            assert_eq!(t.r#type, crate::ir::common::StartStop::Start);
            assert_eq!(t.number, Some(1));
            assert_eq!(t.bracket, Some(YesNo::Yes));
        } else {
            panic!("Expected Tuplet notation");
        }
    } else {
        panic!("Expected Note");
    }
}

// =======================================================================
// Error Message Tests (Milestone 5, Task 5.5)
// =======================================================================

#[test]
fn test_parse_error_missing_required_element() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let result = parse_score(xml);
    assert!(result.is_err());
    let err = result.unwrap_err();
    // Error should mention missing part-list
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("part-list") || err_str.contains("missing"));
}

#[test]
fn test_parse_error_invalid_attribute_value() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>X</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let result = parse_score(xml);
    assert!(result.is_err());
}

// =======================================================================
// Score Header Parsing Tests - parse_work
// =======================================================================

#[test]
fn test_parse_work_with_opus() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <work>
                    <work-number>BWV 1007</work-number>
                    <work-title>Cello Suite No. 1</work-title>
                    <opus xlink:href="http://example.com/bach/suites"/>
                </work>
                <part-list>
                    <score-part id="P1">
                        <part-name>Cello</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert!(score.work.is_some());
    let work = score.work.as_ref().unwrap();
    assert_eq!(work.work_number, Some("BWV 1007".to_string()));
    assert_eq!(work.work_title, Some("Cello Suite No. 1".to_string()));
    assert!(work.opus.is_some());
    assert_eq!(
        work.opus.as_ref().unwrap().href,
        "http://example.com/bach/suites"
    );
}

#[test]
fn test_parse_work_empty_opus() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <work>
                    <work-title>Test Work</work-title>
                    <opus xlink:href="http://example.com/opus"/>
                </work>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert!(score.work.is_some());
    assert!(score.work.as_ref().unwrap().opus.is_some());
}

#[test]
fn test_parse_work_only_title() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <work>
                    <work-title>Untitled Composition</work-title>
                </work>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    let work = score.work.as_ref().unwrap();
    assert!(work.work_number.is_none());
    assert_eq!(work.work_title, Some("Untitled Composition".to_string()));
    assert!(work.opus.is_none());
}

// =======================================================================
// Identification Parsing Tests - parse_identification
// =======================================================================

#[test]
fn test_parse_identification_multiple_creators() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <identification>
                    <creator type="composer">Wolfgang Amadeus Mozart</creator>
                    <creator type="lyricist">Lorenzo Da Ponte</creator>
                    <creator type="arranger">Unknown</creator>
                </identification>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    let id = score.identification.as_ref().unwrap();
    assert_eq!(id.creators.len(), 3);
    assert_eq!(id.creators[0].r#type, Some("composer".to_string()));
    assert_eq!(id.creators[0].value, "Wolfgang Amadeus Mozart");
    assert_eq!(id.creators[1].r#type, Some("lyricist".to_string()));
    assert_eq!(id.creators[2].r#type, Some("arranger".to_string()));
}

#[test]
fn test_parse_identification_with_rights() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <identification>
                    <rights type="copyright">Copyright 2024 Test Publisher</rights>
                    <rights>All rights reserved</rights>
                </identification>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    let id = score.identification.as_ref().unwrap();
    assert_eq!(id.rights.len(), 2);
    assert_eq!(id.rights[0].r#type, Some("copyright".to_string()));
    assert_eq!(id.rights[0].value, "Copyright 2024 Test Publisher");
    assert!(id.rights[1].r#type.is_none());
}

#[test]
fn test_parse_identification_with_relation() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <identification>
                    <relation type="arrangement">Based on BWV 565</relation>
                </identification>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    let id = score.identification.as_ref().unwrap();
    assert_eq!(id.relations.len(), 1);
    assert_eq!(id.relations[0].r#type, Some("arrangement".to_string()));
    assert_eq!(id.relations[0].value, "Based on BWV 565");
}

#[test]
fn test_parse_identification_with_miscellaneous() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <identification>
                    <miscellaneous>
                        <miscellaneous-field name="difficulty">Intermediate</miscellaneous-field>
                        <miscellaneous-field name="genre">Classical</miscellaneous-field>
                    </miscellaneous>
                </identification>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    let id = score.identification.as_ref().unwrap();
    assert!(id.miscellaneous.is_some());
    let misc = id.miscellaneous.as_ref().unwrap();
    assert_eq!(misc.fields.len(), 2);
    assert_eq!(misc.fields[0].name, "difficulty");
    assert_eq!(misc.fields[0].value, "Intermediate");
}

// =======================================================================
// Encoding Parsing Tests - parse_encoding
// =======================================================================

#[test]
fn test_parse_encoding_with_encoder() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <identification>
                    <encoding>
                        <encoder type="transcriber">John Doe</encoder>
                        <encoding-date>2024-06-15</encoding-date>
                        <software>Finale 2023</software>
                        <encoding-description>Transcribed from manuscript</encoding-description>
                    </encoding>
                </identification>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    let encoding = score
        .identification
        .as_ref()
        .unwrap()
        .encoding
        .as_ref()
        .unwrap();

    // Check for encoder
    let has_encoder = encoding.content.iter().any(
        |c| matches!(c, crate::ir::common::EncodingContent::Encoder(e) if e.value == "John Doe"),
    );
    assert!(has_encoder);

    // Check for encoding date
    let has_date = encoding.content.iter().any(
        |c| matches!(c, crate::ir::common::EncodingContent::EncodingDate(d) if d == "2024-06-15"),
    );
    assert!(has_date);

    // Check for encoding description
    let has_desc = encoding.content.iter().any(|c| {
            matches!(c, crate::ir::common::EncodingContent::EncodingDescription(d) if d == "Transcribed from manuscript")
        });
    assert!(has_desc);
}

#[test]
fn test_parse_encoding_supports_with_attribute_value() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <identification>
                    <encoding>
                        <supports element="print" attribute="new-page" type="yes" value="yes"/>
                        <supports element="print" attribute="new-system" type="yes" value="yes"/>
                    </encoding>
                </identification>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    let encoding = score
        .identification
        .as_ref()
        .unwrap()
        .encoding
        .as_ref()
        .unwrap();

    let supports_count = encoding
        .content
        .iter()
        .filter(|c| matches!(c, crate::ir::common::EncodingContent::Supports(_)))
        .count();
    assert_eq!(supports_count, 2);
}

// =======================================================================
// Credit Parsing Tests - parse_credit
// =======================================================================

#[test]
fn test_parse_credit_with_credit_type() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <credit page="1">
                    <credit-type>title</credit-type>
                    <credit-type>page number</credit-type>
                    <credit-words>Sonata in C Major</credit-words>
                </credit>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    assert_eq!(score.credits.len(), 1);

    // Check credit types
    let credit_types: Vec<_> = score.credits[0]
        .content
        .iter()
        .filter_map(|c| {
            if let CreditContent::CreditType(t) = c {
                Some(t.clone())
            } else {
                None
            }
        })
        .collect();
    assert_eq!(credit_types.len(), 2);
    assert!(credit_types.contains(&"title".to_string()));
    assert!(credit_types.contains(&"page number".to_string()));
}

#[test]
fn test_parse_credit_words_with_attributes() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <credit page="1">
                    <credit-words justify="center" halign="center" valign="top" xml:lang="en">Symphony No. 5</credit-words>
                </credit>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let CreditContent::CreditWords(cw) = &score.credits[0].content[0] {
        assert_eq!(cw.value, "Symphony No. 5");
        assert_eq!(cw.justify, Some(crate::ir::common::LeftCenterRight::Center));
        assert_eq!(cw.halign, Some(crate::ir::common::LeftCenterRight::Center));
        assert_eq!(cw.valign, Some(crate::ir::common::TopMiddleBottom::Top));
        assert_eq!(cw.lang, Some("en".to_string()));
    } else {
        panic!("Expected CreditWords");
    }
}

#[test]
fn test_parse_credit_image() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <credit page="1">
                    <credit-image source="logo.png" type="image/png"/>
                </credit>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let CreditContent::CreditImage(img) = &score.credits[0].content[0] {
        assert_eq!(img.source, "logo.png");
        assert_eq!(img.r#type, "image/png");
    } else {
        panic!("Expected CreditImage");
    }
}

#[test]
fn test_parse_empty_credit_words() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <credit page="1">
                    <credit-words/>
                </credit>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let CreditContent::CreditWords(cw) = &score.credits[0].content[0] {
        assert_eq!(cw.value, "");
    } else {
        panic!("Expected CreditWords");
    }
}

// =======================================================================
// Defaults Parsing Tests - parse_defaults
// =======================================================================

#[test]
fn test_parse_defaults_with_system_layout() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <defaults>
                    <system-layout>
                        <system-margins>
                            <left-margin>70</left-margin>
                            <right-margin>70</right-margin>
                        </system-margins>
                        <system-distance>121</system-distance>
                        <top-system-distance>70</top-system-distance>
                    </system-layout>
                </defaults>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    let defaults = score.defaults.as_ref().unwrap();
    assert!(defaults.system_layout.is_some());
    let system_layout = defaults.system_layout.as_ref().unwrap();
    assert!(system_layout.system_margins.is_some());
    let margins = system_layout.system_margins.as_ref().unwrap();
    assert_eq!(margins.left, 70.0);
    assert_eq!(margins.right, 70.0);
    assert_eq!(system_layout.system_distance, Some(121.0));
    assert_eq!(system_layout.top_system_distance, Some(70.0));
}

#[test]
fn test_parse_defaults_with_staff_layout() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <defaults>
                    <staff-layout number="1">
                        <staff-distance>65</staff-distance>
                    </staff-layout>
                    <staff-layout number="2">
                        <staff-distance>75</staff-distance>
                    </staff-layout>
                </defaults>
                <part-list>
                    <score-part id="P1">
                        <part-name>Piano</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    let defaults = score.defaults.as_ref().unwrap();
    assert_eq!(defaults.staff_layout.len(), 2);
    assert_eq!(defaults.staff_layout[0].number, Some(1));
    assert_eq!(defaults.staff_layout[0].staff_distance, Some(65.0));
    assert_eq!(defaults.staff_layout[1].number, Some(2));
}

#[test]
fn test_parse_defaults_with_appearance() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <defaults>
                    <appearance>
                        <line-width type="stem">1.0</line-width>
                        <line-width type="beam">5.0</line-width>
                        <line-width type="staff">0.83</line-width>
                        <note-size type="grace">60</note-size>
                        <note-size type="cue">75</note-size>
                        <distance type="hyphen">60</distance>
                        <distance type="beam">8</distance>
                    </appearance>
                </defaults>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    let defaults = score.defaults.as_ref().unwrap();
    assert!(defaults.appearance.is_some());
    let appearance = defaults.appearance.as_ref().unwrap();
    assert_eq!(appearance.line_widths.len(), 3);
    assert_eq!(appearance.note_sizes.len(), 2);
    assert_eq!(appearance.distances.len(), 2);
}

#[test]
fn test_parse_defaults_with_fonts() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <defaults>
                    <music-font font-family="Bravura" font-size="20"/>
                    <word-font font-family="Times New Roman" font-size="10"/>
                    <lyric-font number="1" font-family="Times New Roman" font-size="11"/>
                    <lyric-language number="1" xml:lang="en"/>
                </defaults>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    let defaults = score.defaults.as_ref().unwrap();
    assert!(defaults.music_font.is_some());
    assert!(defaults.word_font.is_some());
    assert_eq!(defaults.lyric_fonts.len(), 1);
    assert_eq!(defaults.lyric_languages.len(), 1);
    assert_eq!(defaults.lyric_languages[0].lang, "en");
}

#[test]
fn test_parse_defaults_with_system_dividers() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <defaults>
                    <system-layout>
                        <system-dividers>
                            <left-divider print-object="yes"/>
                            <right-divider print-object="no"/>
                        </system-dividers>
                    </system-layout>
                </defaults>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    let defaults = score.defaults.as_ref().unwrap();
    let system_layout = defaults.system_layout.as_ref().unwrap();
    assert!(system_layout.system_dividers.is_some());
    let dividers = system_layout.system_dividers.as_ref().unwrap();
    assert_eq!(
        dividers.left_divider.as_ref().unwrap().print_object,
        Some(YesNo::Yes)
    );
    assert_eq!(
        dividers.right_divider.as_ref().unwrap().print_object,
        Some(YesNo::No)
    );
}

// =======================================================================
// Lyric Parsing Tests - parse_lyric
// =======================================================================

#[test]
fn test_parse_lyric_syllabic_begin() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Voice</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>C</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <lyric number="1">
                                <syllabic>begin</syllabic>
                                <text>Hap</text>
                            </lyric>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let LyricContent::Syllable { syllabic, text, .. } = &note.lyrics[0].content {
            assert_eq!(*syllabic, Some(crate::ir::lyric::Syllabic::Begin));
            assert_eq!(text.value, "Hap");
        } else {
            panic!("Expected Syllable content");
        }
    }
}

#[test]
fn test_parse_lyric_syllabic_middle() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Voice</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>C</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <lyric number="1">
                                <syllabic>middle</syllabic>
                                <text>pi</text>
                            </lyric>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let LyricContent::Syllable { syllabic, .. } = &note.lyrics[0].content {
            assert_eq!(*syllabic, Some(crate::ir::lyric::Syllabic::Middle));
        }
    }
}

#[test]
fn test_parse_lyric_syllabic_end() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Voice</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>C</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <lyric number="1">
                                <syllabic>end</syllabic>
                                <text>ness</text>
                            </lyric>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let LyricContent::Syllable { syllabic, text, .. } = &note.lyrics[0].content {
            assert_eq!(*syllabic, Some(crate::ir::lyric::Syllabic::End));
            assert_eq!(text.value, "ness");
        }
    }
}

#[test]
fn test_parse_lyric_with_elision() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Voice</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>C</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <lyric number="1">
                                <syllabic>single</syllabic>
                                <text>the</text>
                                <elision>_</elision>
                                <syllabic>single</syllabic>
                                <text>a</text>
                            </lyric>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let LyricContent::Syllable { extensions, .. } = &note.lyrics[0].content {
            assert_eq!(extensions.len(), 1);
            assert_eq!(extensions[0].elision.value, "_");
            assert_eq!(extensions[0].text.value, "a");
        }
    }
}

#[test]
fn test_parse_lyric_with_extend_stop() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Voice</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>C</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <lyric number="1">
                                <extend type="stop"/>
                            </lyric>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let LyricContent::ExtendOnly(ext) = &note.lyrics[0].content {
            assert_eq!(ext.r#type, Some(crate::ir::common::StartStopContinue::Stop));
        } else {
            panic!("Expected ExtendOnly content");
        }
    }
}

#[test]
fn test_parse_lyric_text_with_font() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Voice</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>C</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <lyric number="1">
                                <syllabic>single</syllabic>
                                <text font-family="Times" font-style="italic" font-weight="bold" font-size="12">love</text>
                            </lyric>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let LyricContent::Syllable { text, .. } = &note.lyrics[0].content {
            assert_eq!(text.font.font_family, Some("Times".to_string()));
            assert_eq!(
                text.font.font_style,
                Some(crate::ir::common::FontStyle::Italic)
            );
            assert_eq!(
                text.font.font_weight,
                Some(crate::ir::common::FontWeight::Bold)
            );
        }
    }
}

#[test]
fn test_parse_lyric_with_end_line_and_end_paragraph() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Voice</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>C</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <lyric number="1">
                                <syllabic>single</syllabic>
                                <text>word</text>
                                <end-line/>
                                <end-paragraph/>
                            </lyric>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        assert!(note.lyrics[0].end_line);
        assert!(note.lyrics[0].end_paragraph);
    }
}

// =======================================================================
// Direction Parsing Tests - All dynamics types
// =======================================================================

#[test]
fn test_parse_direction_with_other_dynamics() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <direction>
                            <direction-type>
                                <dynamics>
                                    <other-dynamics>molto f</other-dynamics>
                                </dynamics>
                            </direction-type>
                        </direction>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Direction(dir) =
        &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::direction::DirectionTypeContent::Dynamics(d) =
            &dir.direction_types[0].content
        {
            if let crate::ir::direction::DynamicElement::OtherDynamics(text) = &d.content[0] {
                assert_eq!(text, "molto f");
            } else {
                panic!("Expected OtherDynamics");
            }
        }
    }
}

#[test]
fn test_parse_direction_wedge_diminuendo() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <direction>
                            <direction-type>
                                <wedge type="diminuendo" number="1" spread="15"/>
                            </direction-type>
                        </direction>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Direction(dir) =
        &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::direction::DirectionTypeContent::Wedge(w) =
            &dir.direction_types[0].content
        {
            assert_eq!(w.r#type, crate::ir::direction::WedgeType::Diminuendo);
            assert_eq!(w.number, Some(1));
            assert_eq!(w.spread, Some(15.0));
        }
    }
}

#[test]
fn test_parse_direction_wedge_stop() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <direction>
                            <direction-type>
                                <wedge type="stop" number="1"/>
                            </direction-type>
                        </direction>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Direction(dir) =
        &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::direction::DirectionTypeContent::Wedge(w) =
            &dir.direction_types[0].content
        {
            assert_eq!(w.r#type, crate::ir::direction::WedgeType::Stop);
        }
    }
}

#[test]
fn test_parse_direction_metronome_with_dots() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <direction>
                            <direction-type>
                                <metronome parentheses="yes">
                                    <beat-unit>quarter</beat-unit>
                                    <beat-unit-dot/>
                                    <per-minute>72</per-minute>
                                </metronome>
                            </direction-type>
                        </direction>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Direction(dir) =
        &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::direction::DirectionTypeContent::Metronome(m) =
            &dir.direction_types[0].content
        {
            assert_eq!(m.parentheses, Some(YesNo::Yes));
            if let crate::ir::direction::MetronomeContent::PerMinute {
                beat_unit,
                beat_unit_dots,
                per_minute,
                ..
            } = &m.content
            {
                assert_eq!(*beat_unit, crate::ir::duration::NoteTypeValue::Quarter);
                assert_eq!(*beat_unit_dots, 1);
                assert_eq!(per_minute.value, "72");
            }
        }
    }
}

#[test]
fn test_parse_direction_pedal_types() {
    let pedal_types = ["start", "stop", "change", "continue"];
    for pedal_type in pedal_types {
        let xml = format!(
            r#"<?xml version="1.0"?>
                <score-partwise>
                    <part-list>
                        <score-part id="P1">
                            <part-name>Test</part-name>
                        </score-part>
                    </part-list>
                    <part id="P1">
                        <measure number="1">
                            <direction>
                                <direction-type>
                                    <pedal type="{}"/>
                                </direction-type>
                            </direction>
                        </measure>
                    </part>
                </score-partwise>"#,
            pedal_type
        );

        let score = parse_score(&xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Direction(dir) =
            &score.parts[0].measures[0].content[0]
        {
            if let crate::ir::direction::DirectionTypeContent::Pedal(p) =
                &dir.direction_types[0].content
            {
                // Just verify it parses without panic
                let _ = p.r#type;
            }
        }
    }
}

#[test]
fn test_parse_direction_octave_shift() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <direction>
                            <direction-type>
                                <octave-shift type="up" size="8"/>
                            </direction-type>
                        </direction>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Direction(dir) =
        &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::direction::DirectionTypeContent::OctaveShift(os) =
            &dir.direction_types[0].content
        {
            assert_eq!(os.r#type, crate::ir::direction::UpDownStopContinue::Up);
            assert_eq!(os.size, Some(8));
        }
    }
}

#[test]
fn test_parse_direction_dashes() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <direction>
                            <direction-type>
                                <dashes type="start" number="1"/>
                            </direction-type>
                        </direction>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Direction(dir) =
        &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::direction::DirectionTypeContent::Dashes(d) =
            &dir.direction_types[0].content
        {
            assert_eq!(d.r#type, crate::ir::common::StartStopContinue::Start);
            assert_eq!(d.number, Some(1));
        }
    }
}

#[test]
fn test_parse_direction_bracket() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <direction>
                            <direction-type>
                                <bracket type="start" number="1" line-end="up" line-type="solid"/>
                            </direction-type>
                        </direction>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Direction(dir) =
        &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::direction::DirectionTypeContent::Bracket(b) =
            &dir.direction_types[0].content
        {
            assert_eq!(b.r#type, crate::ir::common::StartStopContinue::Start);
            assert_eq!(b.line_end, crate::ir::direction::LineEnd::Up);
        }
    }
}

#[test]
fn test_parse_direction_with_offset() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <direction>
                            <direction-type>
                                <dynamics><f/></dynamics>
                            </direction-type>
                            <offset sound="yes">-2</offset>
                        </direction>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Direction(dir) =
        &score.parts[0].measures[0].content[0]
    {
        assert!(dir.offset.is_some());
        let offset = dir.offset.as_ref().unwrap();
        assert_eq!(offset.value, -2);
        assert_eq!(offset.sound, Some(YesNo::Yes));
    }
}

#[test]
fn test_parse_direction_with_rehearsal() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <direction>
                            <direction-type>
                                <rehearsal>A</rehearsal>
                            </direction-type>
                        </direction>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Direction(dir) =
        &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::direction::DirectionTypeContent::Rehearsal(r) =
            &dir.direction_types[0].content
        {
            assert_eq!(r.len(), 1);
            assert_eq!(r[0].value, "A");
        }
    }
}

#[test]
fn test_parse_direction_segno_and_coda() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <direction>
                            <direction-type>
                                <segno/>
                            </direction-type>
                        </direction>
                        <direction>
                            <direction-type>
                                <coda/>
                            </direction-type>
                        </direction>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    // Check segno
    if let crate::ir::measure::MusicDataElement::Direction(dir) =
        &score.parts[0].measures[0].content[0]
    {
        assert!(matches!(
            &dir.direction_types[0].content,
            crate::ir::direction::DirectionTypeContent::Segno(_)
        ));
    }
    // Check coda
    if let crate::ir::measure::MusicDataElement::Direction(dir) =
        &score.parts[0].measures[0].content[1]
    {
        assert!(matches!(
            &dir.direction_types[0].content,
            crate::ir::direction::DirectionTypeContent::Coda(_)
        ));
    }
}

// =======================================================================
// Notation Parsing Tests - Tied, Slur, Tuplet
// =======================================================================

#[test]
fn test_parse_tied_with_attributes() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>C</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <tied type="start" number="1" orientation="over"/>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Tied(t) = &note.notations[0].content[0] {
            assert_eq!(t.r#type, crate::ir::common::StartStopContinue::Start);
            assert_eq!(t.number, Some(1));
            assert_eq!(t.orientation, Some(crate::ir::common::OverUnder::Over));
        }
    }
}

#[test]
fn test_parse_slur_with_bezier() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>C</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <slur type="start" number="1" placement="above" bezier-x="10" bezier-y="20"/>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Slur(s) = &note.notations[0].content[0] {
            assert_eq!(s.r#type, crate::ir::common::StartStopContinue::Start);
            assert_eq!(s.placement, Some(crate::ir::common::AboveBelow::Above));
        }
    }
}

#[test]
fn test_parse_tuplet_with_actual_normal_notes() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>C</step><octave>4</octave></pitch>
                            <duration>2</duration>
                            <type>eighth</type>
                            <time-modification>
                                <actual-notes>5</actual-notes>
                                <normal-notes>4</normal-notes>
                            </time-modification>
                            <notations>
                                <tuplet type="start" number="1" show-number="both" show-type="actual">
                                    <tuplet-actual>
                                        <tuplet-number>5</tuplet-number>
                                        <tuplet-type>eighth</tuplet-type>
                                    </tuplet-actual>
                                    <tuplet-normal>
                                        <tuplet-number>4</tuplet-number>
                                        <tuplet-type>eighth</tuplet-type>
                                    </tuplet-normal>
                                </tuplet>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        assert!(note.time_modification.is_some());
        let tm = note.time_modification.as_ref().unwrap();
        assert_eq!(tm.actual_notes, 5);
        assert_eq!(tm.normal_notes, 4);

        if let crate::ir::notation::NotationContent::Tuplet(t) = &note.notations[0].content[0] {
            assert!(t.tuplet_actual.is_some());
            assert!(t.tuplet_normal.is_some());
            let actual = t.tuplet_actual.as_ref().unwrap();
            assert_eq!(actual.tuplet_number.as_ref().unwrap().value, 5);
        }
    }
}

// =======================================================================
// Ornament Parsing Tests
// =======================================================================

#[test]
fn test_parse_ornaments_turn_variants() {
    let ornaments = [
        "turn",
        "delayed-turn",
        "inverted-turn",
        "delayed-inverted-turn",
        "vertical-turn",
    ];
    for ornament in ornaments {
        let xml = format!(
            r#"<?xml version="1.0"?>
                <score-partwise>
                    <part-list>
                        <score-part id="P1">
                            <part-name>Test</part-name>
                        </score-part>
                    </part-list>
                    <part id="P1">
                        <measure number="1">
                            <note>
                                <pitch><step>C</step><octave>4</octave></pitch>
                                <duration>4</duration>
                                <type>quarter</type>
                                <notations>
                                    <ornaments>
                                        <{}/>
                                    </ornaments>
                                </notations>
                            </note>
                        </measure>
                    </part>
                </score-partwise>"#,
            ornament
        );

        let result = parse_score(&xml);
        assert!(result.is_ok(), "Failed to parse ornament: {}", ornament);
    }
}

#[test]
fn test_parse_ornaments_mordent() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>C</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <ornaments>
                                    <mordent long="yes"/>
                                </ornaments>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Ornaments(o) = &note.notations[0].content[0] {
            if let crate::ir::notation::OrnamentElement::Mordent(m) = &o.content[0].ornament {
                assert_eq!(m.long, Some(YesNo::Yes));
            }
        }
    }
}

#[test]
fn test_parse_ornaments_inverted_mordent() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>C</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <ornaments>
                                    <inverted-mordent/>
                                </ornaments>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Ornaments(o) = &note.notations[0].content[0] {
            assert!(matches!(
                &o.content[0].ornament,
                crate::ir::notation::OrnamentElement::InvertedMordent(_)
            ));
        }
    }
}

#[test]
fn test_parse_ornaments_shake() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>C</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <ornaments>
                                    <shake/>
                                </ornaments>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Ornaments(o) = &note.notations[0].content[0] {
            assert!(matches!(
                &o.content[0].ornament,
                crate::ir::notation::OrnamentElement::Shake(_)
            ));
        }
    }
}

#[test]
fn test_parse_ornaments_tremolo() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>C</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <ornaments>
                                    <tremolo type="single">3</tremolo>
                                </ornaments>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Ornaments(o) = &note.notations[0].content[0] {
            if let crate::ir::notation::OrnamentElement::Tremolo(t) = &o.content[0].ornament {
                assert_eq!(t.value, 3);
                assert_eq!(t.r#type, Some(crate::ir::notation::TremoloType::Single));
            }
        }
    }
}

#[test]
fn test_parse_ornaments_schleifer() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>C</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <ornaments>
                                    <schleifer/>
                                </ornaments>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Ornaments(o) = &note.notations[0].content[0] {
            assert!(matches!(
                &o.content[0].ornament,
                crate::ir::notation::OrnamentElement::Schleifer(_)
            ));
        }
    }
}

#[test]
fn test_parse_ornaments_with_accidental_mark() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>C</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <ornaments>
                                    <trill-mark/>
                                    <accidental-mark>sharp</accidental-mark>
                                </ornaments>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Ornaments(o) = &note.notations[0].content[0] {
            assert!(!o.content[0].accidental_marks.is_empty());
        }
    }
}

// =======================================================================
// Articulation Parsing Tests
// =======================================================================

#[test]
fn test_parse_articulations_all_basic_types() {
    let articulations = [
        "staccato",
        "tenuto",
        "detached-legato",
        "staccatissimo",
        "spiccato",
        "accent",
    ];
    for artic in articulations {
        let xml = format!(
            r#"<?xml version="1.0"?>
                <score-partwise>
                    <part-list>
                        <score-part id="P1">
                            <part-name>Test</part-name>
                        </score-part>
                    </part-list>
                    <part id="P1">
                        <measure number="1">
                            <note>
                                <pitch><step>C</step><octave>4</octave></pitch>
                                <duration>4</duration>
                                <type>quarter</type>
                                <notations>
                                    <articulations>
                                        <{}/>
                                    </articulations>
                                </notations>
                            </note>
                        </measure>
                    </part>
                </score-partwise>"#,
            artic
        );

        let result = parse_score(&xml);
        assert!(result.is_ok(), "Failed to parse articulation: {}", artic);
    }
}

#[test]
fn test_parse_articulations_strong_accent() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>C</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <articulations>
                                    <strong-accent type="up"/>
                                </articulations>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Articulations(a) =
            &note.notations[0].content[0]
        {
            if let crate::ir::notation::ArticulationElement::StrongAccent(sa) = &a.content[0] {
                assert_eq!(sa.r#type, Some(crate::ir::common::UpDown::Up));
            }
        }
    }
}

#[test]
fn test_parse_articulations_breath_mark() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>C</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <articulations>
                                    <breath-mark>comma</breath-mark>
                                </articulations>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Articulations(a) =
            &note.notations[0].content[0]
        {
            if let crate::ir::notation::ArticulationElement::BreathMark(bm) = &a.content[0] {
                assert_eq!(bm.value, crate::ir::notation::BreathMarkValue::Comma);
            }
        }
    }
}

#[test]
fn test_parse_articulations_caesura() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>C</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <articulations>
                                    <caesura>normal</caesura>
                                </articulations>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Articulations(a) =
            &note.notations[0].content[0]
        {
            assert!(matches!(
                &a.content[0],
                crate::ir::notation::ArticulationElement::Caesura(_)
            ));
        }
    }
}

#[test]
fn test_parse_articulations_scoop_plop_doit_falloff() {
    let jazz_articulations = ["scoop", "plop", "doit", "falloff"];
    for artic in jazz_articulations {
        let xml = format!(
            r#"<?xml version="1.0"?>
                <score-partwise>
                    <part-list>
                        <score-part id="P1">
                            <part-name>Test</part-name>
                        </score-part>
                    </part-list>
                    <part id="P1">
                        <measure number="1">
                            <note>
                                <pitch><step>C</step><octave>4</octave></pitch>
                                <duration>4</duration>
                                <type>quarter</type>
                                <notations>
                                    <articulations>
                                        <{}/>
                                    </articulations>
                                </notations>
                            </note>
                        </measure>
                    </part>
                </score-partwise>"#,
            artic
        );

        let result = parse_score(&xml);
        assert!(result.is_ok(), "Failed to parse articulation: {}", artic);
    }
}

// =======================================================================
// Technical Parsing Tests
// =======================================================================

#[test]
fn test_parse_technical_up_bow_down_bow() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Violin</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>G</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <technical>
                                    <up-bow/>
                                </technical>
                            </notations>
                        </note>
                        <note>
                            <pitch><step>A</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <technical>
                                    <down-bow/>
                                </technical>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Technical(t) = &note.notations[0].content[0] {
            assert!(matches!(
                &t.content[0],
                crate::ir::notation::TechnicalElement::UpBow(_)
            ));
        }
    }
}

#[test]
fn test_parse_technical_string_and_fret() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Guitar</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>E</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <technical>
                                    <string>1</string>
                                    <fret>0</fret>
                                </technical>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Technical(t) = &note.notations[0].content[0] {
            assert_eq!(t.content.len(), 2);
        }
    }
}

#[test]
fn test_parse_technical_hammer_on_pull_off() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Guitar</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>E</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <technical>
                                    <hammer-on type="start" number="1">H</hammer-on>
                                </technical>
                            </notations>
                        </note>
                        <note>
                            <pitch><step>F</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <technical>
                                    <pull-off type="stop" number="1">P</pull-off>
                                </technical>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Technical(t) = &note.notations[0].content[0] {
            if let crate::ir::notation::TechnicalElement::HammerOn(h) = &t.content[0] {
                assert_eq!(h.r#type, crate::ir::common::StartStop::Start);
                assert_eq!(h.value, "H");
            }
        }
    }
}

#[test]
fn test_parse_technical_harmonic() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Violin</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>E</step><octave>5</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <technical>
                                    <harmonic>
                                        <natural/>
                                        <touching-pitch/>
                                    </harmonic>
                                </technical>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Technical(t) = &note.notations[0].content[0] {
            if let crate::ir::notation::TechnicalElement::Harmonic(h) = &t.content[0] {
                assert!(h.natural);
                assert!(h.touching_pitch);
            }
        }
    }
}

#[test]
fn test_parse_technical_bend() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Guitar</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>D</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <technical>
                                    <bend>
                                        <bend-alter>2</bend-alter>
                                        <release/>
                                    </bend>
                                </technical>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Technical(t) = &note.notations[0].content[0] {
            if let crate::ir::notation::TechnicalElement::Bend(b) = &t.content[0] {
                assert_eq!(b.bend_alter, 2.0);
                assert!(b.release.is_some());
            }
        }
    }
}

#[test]
fn test_parse_technical_pluck_and_tap() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Guitar</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>E</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <technical>
                                    <pluck>p</pluck>
                                </technical>
                            </notations>
                        </note>
                        <note>
                            <pitch><step>E</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <technical>
                                    <tap>T</tap>
                                </technical>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Technical(t) = &note.notations[0].content[0] {
            if let crate::ir::notation::TechnicalElement::Pluck(p) = &t.content[0] {
                assert_eq!(p.value, "p");
            }
        }
    }
}

// =======================================================================
// Barline Parsing Tests
// =======================================================================

#[test]
fn test_parse_barline_fermata_shapes() {
    let shapes = [
        "normal",
        "angled",
        "square",
        "double-angled",
        "double-square",
        "double-dot",
        "half-curve",
        "curlew",
    ];
    for shape in shapes {
        let xml = format!(
            r#"<?xml version="1.0"?>
                <score-partwise>
                    <part-list>
                        <score-part id="P1">
                            <part-name>Test</part-name>
                        </score-part>
                    </part-list>
                    <part id="P1">
                        <measure number="1">
                            <barline location="right">
                                <fermata type="upright">{}</fermata>
                            </barline>
                        </measure>
                    </part>
                </score-partwise>"#,
            shape
        );

        let result = parse_score(&xml);
        assert!(
            result.is_ok(),
            "Failed to parse fermata shape: {} - {:?}",
            shape,
            result.err()
        );
    }
}

#[test]
fn test_parse_barline_wavy_line_continue() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline>
                            <wavy-line type="continue" number="1"/>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Barline(barline) =
        &score.parts[0].measures[0].content[0]
    {
        let wavy = barline.wavy_line.as_ref().unwrap();
        assert_eq!(wavy.r#type, crate::ir::common::StartStopContinue::Continue);
    }
}

#[test]
fn test_parse_barline_ending_print_object() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline location="left">
                            <ending number="1" type="start" print-object="no"/>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Barline(barline) =
        &score.parts[0].measures[0].content[0]
    {
        let ending = barline.ending.as_ref().unwrap();
        assert_eq!(ending.print_object, Some(YesNo::No));
    }
}

#[test]
fn test_parse_barline_repeat_backward() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline location="right">
                            <repeat direction="backward"/>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Barline(barline) =
        &score.parts[0].measures[0].content[0]
    {
        let repeat = barline.repeat.as_ref().unwrap();
        assert_eq!(
            repeat.direction,
            crate::ir::common::BackwardForward::Backward
        );
    }
}

// =======================================================================
// Glissando and Slide Tests
// =======================================================================

#[test]
fn test_parse_glissando() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>C</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <glissando type="start" number="1" line-type="wavy">gliss.</glissando>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Glissando(g) = &note.notations[0].content[0] {
            assert_eq!(g.r#type, crate::ir::common::StartStop::Start);
            assert_eq!(g.text, Some("gliss.".to_string()));
            assert_eq!(g.line_type, Some(crate::ir::common::LineType::Wavy));
        }
    }
}

#[test]
fn test_parse_slide() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>C</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <slide type="start" number="1">slide</slide>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::Slide(s) = &note.notations[0].content[0] {
            assert_eq!(s.r#type, crate::ir::common::StartStop::Start);
            assert_eq!(s.text, Some("slide".to_string()));
        }
    }
}

// =======================================================================
// Non-Arpeggiate Test
// =======================================================================

#[test]
fn test_parse_non_arpeggiate() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>C</step><octave>4</octave></pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <notations>
                                <non-arpeggiate type="bottom"/>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        if let crate::ir::notation::NotationContent::NonArpeggiate(na) =
            &note.notations[0].content[0]
        {
            assert_eq!(na.r#type, crate::ir::notation::TopBottom::Bottom);
        }
    }
}

// =======================================================================
// Transpose Test
// =======================================================================

#[test]
fn test_parse_transpose() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Bb Clarinet</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <attributes>
                            <transpose>
                                <diatonic>-1</diatonic>
                                <chromatic>-2</chromatic>
                            </transpose>
                        </attributes>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Attributes(attrs) =
        &score.parts[0].measures[0].content[0]
    {
        assert_eq!(attrs.transpose.len(), 1);
        assert_eq!(attrs.transpose[0].diatonic, Some(-1));
        assert_eq!(attrs.transpose[0].chromatic, -2);
    }
}

// =======================================================================
// Time Modification with normal-type and normal-dot
// =======================================================================

#[test]
fn test_parse_time_modification_with_normal_type() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch><step>C</step><octave>4</octave></pitch>
                            <duration>2</duration>
                            <type>16th</type>
                            <time-modification>
                                <actual-notes>6</actual-notes>
                                <normal-notes>4</normal-notes>
                                <normal-type>16th</normal-type>
                                <normal-dot/>
                            </time-modification>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Note(note) = &score.parts[0].measures[0].content[0]
    {
        let tm = note.time_modification.as_ref().unwrap();
        assert_eq!(tm.actual_notes, 6);
        assert_eq!(tm.normal_notes, 4);
        assert!(tm.normal_type.is_some());
        assert_eq!(tm.normal_dots, 1);
    }
}

// =======================================================================
// Empty Clef Test
// =======================================================================

#[test]
fn test_parse_empty_clef() {
    let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <attributes>
                            <clef number="1"/>
                        </attributes>
                    </measure>
                </part>
            </score-partwise>"#;

    let score = parse_score(xml).unwrap();
    if let crate::ir::measure::MusicDataElement::Attributes(attrs) =
        &score.parts[0].measures[0].content[0]
    {
        assert_eq!(attrs.clefs.len(), 1);
        assert_eq!(attrs.clefs[0].number, Some(1));
    }
}

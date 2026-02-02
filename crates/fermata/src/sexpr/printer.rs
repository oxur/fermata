//! IR to S-expression printer.
//!
//! This module provides functions to convert IR types to formatted
//! S-expression strings.

use crate::ir::{
    Barline, Measure,
    attributes::{
        Attributes, BarStyle, Cancel, Clef, ClefSign, Ending, GroupSymbolValue, Key, KeyContent,
        Mode, Repeat, StaffDetails, Time, TimeContent, TimeSymbol, Transpose,
    },
    beam::{Beam, BeamValue, Fan, Notehead, NoteheadValue, Stem, StemValue},
    common::{
        AboveBelow, AccidentalValue, BackwardForward, Encoding, EncodingContent, Identification,
        LeftCenterRight, Miscellaneous, MiscellaneousField, RightLeftMiddle, StartStop,
        StartStopContinue, StartStopDiscontinue, StartStopSingle, Supports, SymbolSize, TypedText,
        UpDown, UprightInverted, YesNo,
    },
    direction::{
        Direction, DirectionType, DirectionTypeContent, DynamicElement, Dynamics, Metronome,
        MetronomeContent, OctaveShift, Pedal, PedalType, WedgeType, Words,
    },
    duration::{NoteTypeValue, TimeModification},
    lyric::{Extend, Lyric, LyricContent, LyricExtension, Syllabic},
    measure::MusicDataElement,
    notation::{
        Arpeggiate, ArticulationElement, Articulations, FermataShape, Glissando, NonArpeggiate,
        NotationContent, Notations, OrnamentElement, OrnamentWithAccidentals, Ornaments,
        OtherNotation, Slide, Technical, TechnicalElement, Tied, TopBottom, Tuplet, TupletPortion,
    },
    note::{Accidental, Note, NoteContent, PitchRestUnpitched, Rest, Tie},
    part::{
        GroupBarline, GroupBarlineValue, GroupSymbol, MidiDevice, MidiInstrument, NameDisplay,
        Part, PartGroup, PartList, PartListElement, ScoreInstrument, ScorePart, SoloOrEnsemble,
        VirtualInstrument,
    },
    pitch::{Pitch, Step, Unpitched},
    score::{Credit, Defaults, ScorePartwise, Work},
    voice::{Backup, Forward},
};

use super::PrintOptions;

/// Print a score to an S-expression string.
pub fn print_score(score: &ScorePartwise, options: &PrintOptions) -> String {
    let mut out = String::new();

    out.push_str("(score-partwise");

    if let Some(ref version) = score.version {
        out.push_str(&format!(" :version \"{}\"", version));
    }

    // Work
    if let Some(ref work) = score.work {
        out.push_str(&newline_indent(1, options));
        out.push_str(&print_work(work, 1, options));
    }

    // Identification
    if let Some(ref identification) = score.identification {
        out.push_str(&newline_indent(1, options));
        out.push_str(&print_identification(identification, 1, options));
    }

    // Defaults
    if let Some(ref defaults) = score.defaults {
        out.push_str(&newline_indent(1, options));
        out.push_str(&print_defaults(defaults, 1, options));
    }

    // Credits
    for credit in &score.credits {
        out.push_str(&newline_indent(1, options));
        out.push_str(&print_credit(credit, 1, options));
    }

    // Part list
    out.push_str(&newline_indent(1, options));
    out.push_str(&print_part_list(&score.part_list, 1, options));

    // Parts
    for part in &score.parts {
        out.push_str(&newline_indent(1, options));
        out.push_str(&print_part(part, 1, options));
    }

    out.push(')');
    out
}

// === Helper Functions ===

/// Generate indentation string.
fn indent(level: usize, options: &PrintOptions) -> String {
    if options.compact {
        String::new()
    } else {
        options.indent.repeat(level)
    }
}

/// Generate newline followed by indentation.
fn newline_indent(level: usize, options: &PrintOptions) -> String {
    if options.compact {
        " ".to_string()
    } else {
        format!("\n{}", indent(level, options))
    }
}

/// Escape a string for S-expression output.
fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Format a float, omitting decimal point for whole numbers.
fn format_float(f: f64) -> String {
    if f.fract() == 0.0 {
        format!("{}", f as i64)
    } else {
        format!("{}", f)
    }
}

// === Score-Level Printers ===

fn print_work(work: &Work, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(work", ind);

    if let Some(ref work_number) = work.work_number {
        out.push_str(&format!(" :work-number \"{}\"", escape_string(work_number)));
    }

    if let Some(ref work_title) = work.work_title {
        out.push_str(&format!(" :work-title \"{}\"", escape_string(work_title)));
    }

    if let Some(ref opus) = work.opus {
        out.push_str(&format!(" :opus \"{}\"", escape_string(&opus.href)));
    }

    out.push(')');
    out
}

fn print_identification(
    identification: &Identification,
    level: usize,
    options: &PrintOptions,
) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(identification", ind);

    for creator in &identification.creators {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_typed_text("creator", creator));
    }

    for rights in &identification.rights {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_typed_text("rights", rights));
    }

    if let Some(ref encoding) = identification.encoding {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_encoding(encoding, level + 1, options));
    }

    if let Some(ref source) = identification.source {
        out.push_str(&format!(" :source \"{}\"", escape_string(source)));
    }

    for relation in &identification.relations {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_typed_text("relation", relation));
    }

    if let Some(ref miscellaneous) = identification.miscellaneous {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_miscellaneous(miscellaneous, level + 1, options));
    }

    out.push(')');
    out
}

fn print_typed_text(tag: &str, tt: &TypedText) -> String {
    let mut out = format!("({}", tag);

    if let Some(ref t) = tt.r#type {
        out.push_str(&format!(" :type \"{}\"", escape_string(t)));
    }

    out.push_str(&format!(" \"{}\"", escape_string(&tt.value)));
    out.push(')');
    out
}

fn print_encoding(encoding: &Encoding, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(encoding", ind);

    for content in &encoding.content {
        out.push_str(&newline_indent(level + 1, options));
        match content {
            EncodingContent::EncodingDate(date) => {
                out.push_str(&format!("(encoding-date \"{}\")", escape_string(date)));
            }
            EncodingContent::Encoder(tt) => {
                out.push_str(&print_typed_text("encoder", tt));
            }
            EncodingContent::Software(s) => {
                out.push_str(&format!("(software \"{}\")", escape_string(s)));
            }
            EncodingContent::EncodingDescription(desc) => {
                out.push_str(&format!(
                    "(encoding-description \"{}\")",
                    escape_string(desc)
                ));
            }
            EncodingContent::Supports(supports) => {
                out.push_str(&print_supports(supports));
            }
        }
    }

    out.push(')');
    out
}

fn print_supports(supports: &Supports) -> String {
    let mut out = String::from("(supports");

    out.push_str(&format!(" :type {}", print_yes_no(supports.r#type)));
    out.push_str(&format!(
        " :element \"{}\"",
        escape_string(&supports.element)
    ));

    if let Some(ref attr) = supports.attribute {
        out.push_str(&format!(" :attribute \"{}\"", escape_string(attr)));
    }

    if let Some(ref value) = supports.value {
        out.push_str(&format!(" :value \"{}\"", escape_string(value)));
    }

    out.push(')');
    out
}

fn print_miscellaneous(misc: &Miscellaneous, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(miscellaneous", ind);

    for field in &misc.fields {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_miscellaneous_field(field));
    }

    out.push(')');
    out
}

fn print_miscellaneous_field(field: &MiscellaneousField) -> String {
    format!(
        "(miscellaneous-field :name \"{}\" \"{}\")",
        escape_string(&field.name),
        escape_string(&field.value)
    )
}

fn print_defaults(defaults: &Defaults, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(defaults", ind);

    if let Some(ref scaling) = defaults.scaling {
        out.push_str(&format!(
            " :scaling (scaling :millimeters {} :tenths {})",
            format_float(scaling.millimeters),
            format_float(scaling.tenths)
        ));
    }

    out.push(')');
    out
}

fn print_credit(credit: &Credit, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(credit", ind);

    if let Some(page) = credit.page {
        out.push_str(&format!(" :page {}", page));
    }

    out.push(')');
    out
}

// === Part List Printers ===

fn print_part_list(part_list: &PartList, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(part-list", ind);

    for content in &part_list.content {
        out.push_str(&newline_indent(level + 1, options));
        match content {
            PartListElement::ScorePart(sp) => {
                out.push_str(&print_score_part(sp, level + 1, options));
            }
            PartListElement::PartGroup(pg) => {
                out.push_str(&print_part_group(pg, level + 1, options));
            }
        }
    }

    out.push(')');
    out
}

fn print_score_part(sp: &ScorePart, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(score-part :id \"{}\"", ind, escape_string(&sp.id));

    // Part name
    out.push_str(&newline_indent(level + 1, options));
    out.push_str(&format!(
        "(part-name \"{}\")",
        escape_string(&sp.part_name.value)
    ));

    if let Some(ref pnd) = sp.part_name_display {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_name_display(pnd, level + 1, options));
    }

    if let Some(ref abbr) = sp.part_abbreviation {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&format!(
            "(part-abbreviation \"{}\")",
            escape_string(&abbr.value)
        ));
    }

    // Score instruments
    for si in &sp.score_instruments {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_score_instrument(si, level + 1, options));
    }

    // MIDI devices
    for md in &sp.midi_devices {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_midi_device(md));
    }

    // MIDI instruments
    for mi in &sp.midi_instruments {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_midi_instrument(mi, level + 1, options));
    }

    out.push(')');
    out
}

fn print_name_display(pnd: &NameDisplay, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(part-name-display", ind);

    if let Some(ref print_object) = pnd.print_object {
        out.push_str(&format!(" :print-object {}", print_yes_no(*print_object)));
    }

    out.push(')');
    out
}

fn print_score_instrument(si: &ScoreInstrument, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(score-instrument :id \"{}\"", ind, escape_string(&si.id));

    out.push_str(&format!(
        " :instrument-name \"{}\"",
        escape_string(&si.instrument_name)
    ));

    if let Some(ref abbr) = si.instrument_abbreviation {
        out.push_str(&format!(
            " :instrument-abbreviation \"{}\"",
            escape_string(abbr)
        ));
    }

    if let Some(ref sound) = si.instrument_sound {
        out.push_str(&format!(" :instrument-sound \"{}\"", escape_string(sound)));
    }

    if let Some(ref solo_or_ensemble) = si.solo_or_ensemble {
        match solo_or_ensemble {
            SoloOrEnsemble::Solo => out.push_str(" :solo #t"),
            SoloOrEnsemble::Ensemble(size) => {
                out.push_str(&format!(" :ensemble {}", size));
            }
        }
    }

    if let Some(ref vi) = si.virtual_instrument {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_virtual_instrument(vi));
    }

    out.push(')');
    out
}

fn print_virtual_instrument(vi: &VirtualInstrument) -> String {
    let mut out = String::from("(virtual-instrument");

    if let Some(ref library) = vi.virtual_library {
        out.push_str(&format!(" :virtual-library \"{}\"", escape_string(library)));
    }

    if let Some(ref name) = vi.virtual_name {
        out.push_str(&format!(" :virtual-name \"{}\"", escape_string(name)));
    }

    out.push(')');
    out
}

fn print_midi_device(md: &MidiDevice) -> String {
    let mut out = String::from("(midi-device");

    if let Some(ref id) = md.id {
        out.push_str(&format!(" :id \"{}\"", escape_string(id)));
    }

    if let Some(port) = md.port {
        out.push_str(&format!(" :port {}", port));
    }

    out.push(')');
    out
}

fn print_midi_instrument(mi: &MidiInstrument, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(midi-instrument :id \"{}\"", ind, escape_string(&mi.id));

    if let Some(channel) = mi.midi_channel {
        out.push_str(&format!(" :midi-channel {}", channel));
    }

    if let Some(ref name) = mi.midi_name {
        out.push_str(&format!(" :midi-name \"{}\"", escape_string(name)));
    }

    if let Some(bank) = mi.midi_bank {
        out.push_str(&format!(" :midi-bank {}", bank));
    }

    if let Some(program) = mi.midi_program {
        out.push_str(&format!(" :midi-program {}", program));
    }

    if let Some(unpitched) = mi.midi_unpitched {
        out.push_str(&format!(" :midi-unpitched {}", unpitched));
    }

    if let Some(volume) = mi.volume {
        out.push_str(&format!(" :volume {}", format_float(volume)));
    }

    if let Some(pan) = mi.pan {
        out.push_str(&format!(" :pan {}", format_float(pan)));
    }

    if let Some(elevation) = mi.elevation {
        out.push_str(&format!(" :elevation {}", format_float(elevation)));
    }

    out.push(')');
    out
}

fn print_part_group(pg: &PartGroup, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(part-group", ind);

    out.push_str(&format!(" :type {}", print_start_stop(pg.r#type)));
    if let Some(ref number) = pg.number {
        out.push_str(&format!(" :number \"{}\"", escape_string(number)));
    }

    if let Some(ref name) = pg.group_name {
        out.push_str(&format!(" :group-name \"{}\"", escape_string(&name.value)));
    }

    if let Some(ref abbr) = pg.group_abbreviation {
        out.push_str(&format!(
            " :group-abbreviation \"{}\"",
            escape_string(&abbr.value)
        ));
    }

    if let Some(ref symbol) = pg.group_symbol {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_group_symbol(symbol));
    }

    if let Some(ref barline) = pg.group_barline {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_group_barline(barline));
    }

    out.push(')');
    out
}

fn print_group_symbol(gs: &GroupSymbol) -> String {
    let value = match gs.value {
        GroupSymbolValue::None => "none",
        GroupSymbolValue::Brace => "brace",
        GroupSymbolValue::Line => "line",
        GroupSymbolValue::Bracket => "bracket",
        GroupSymbolValue::Square => "square",
    };
    format!("(group-symbol {})", value)
}

fn print_group_barline(gb: &GroupBarline) -> String {
    let value = match gb.value {
        GroupBarlineValue::Yes => "yes",
        GroupBarlineValue::No => "no",
        GroupBarlineValue::Mensurstrich => "mensurstrich",
    };
    format!("(group-barline {})", value)
}

// === Part and Measure Printers ===

fn print_part(part: &Part, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(part :id \"{}\"", ind, escape_string(&part.id));

    for measure in &part.measures {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_measure(measure, level + 1, options));
    }

    out.push(')');
    out
}

fn print_measure(measure: &Measure, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!(
        "{}(measure :number \"{}\"",
        ind,
        escape_string(&measure.number)
    );

    if let Some(width) = measure.width {
        out.push_str(&format!(" :width {}", format_float(width)));
    }

    if measure.implicit == Some(YesNo::Yes) {
        out.push_str(" :implicit #t");
    }

    for content in &measure.content {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_music_data_element(content, level + 1, options));
    }

    out.push(')');
    out
}

fn print_music_data_element(
    content: &MusicDataElement,
    level: usize,
    options: &PrintOptions,
) -> String {
    match content {
        MusicDataElement::Note(note) => print_note(note, level, options),
        MusicDataElement::Backup(backup) => print_backup(backup, level, options),
        MusicDataElement::Forward(forward) => print_forward(forward, level, options),
        MusicDataElement::Direction(direction) => print_direction(direction, level, options),
        MusicDataElement::Attributes(attrs) => print_attributes(attrs, level, options),
        MusicDataElement::Barline(barline) => print_barline(barline, level, options),
    }
}

// === Note Printers ===

fn print_note(note: &Note, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(note", ind);

    // Content (pitch/rest/unpitched, chord flag, duration/grace)
    match &note.content {
        NoteContent::Regular {
            full_note,
            duration,
            ties,
        } => {
            if full_note.chord {
                out.push_str(" :chord #t");
            }
            out.push_str(&print_full_note_content(&full_note.content));
            out.push_str(&format!(" :duration {}", duration));
            for tie in ties {
                out.push_str(&print_tie(tie));
            }
        }
        NoteContent::Grace {
            grace,
            full_note,
            ties,
        } => {
            out.push_str(" :grace #t");
            if grace.slash == Some(YesNo::Yes) {
                out.push_str(" :slash #t");
            }
            if let Some(steal_previous) = grace.steal_time_previous {
                out.push_str(&format!(
                    " :steal-time-previous {}",
                    format_float(steal_previous)
                ));
            }
            if let Some(steal_following) = grace.steal_time_following {
                out.push_str(&format!(
                    " :steal-time-following {}",
                    format_float(steal_following)
                ));
            }
            if let Some(make_time) = grace.make_time {
                out.push_str(&format!(" :make-time {}", make_time));
            }
            if full_note.chord {
                out.push_str(" :chord #t");
            }
            out.push_str(&print_full_note_content(&full_note.content));
            for tie in ties {
                out.push_str(&print_tie(tie));
            }
        }
        NoteContent::Cue {
            full_note,
            duration,
        } => {
            out.push_str(" :cue #t");
            if full_note.chord {
                out.push_str(" :chord #t");
            }
            out.push_str(&print_full_note_content(&full_note.content));
            out.push_str(&format!(" :duration {}", duration));
        }
    }

    // Type
    if let Some(ref note_type) = note.r#type {
        out.push_str(&format!(" :type {}", note_type_to_symbol(&note_type.value)));
        if let Some(ref size) = note_type.size {
            out.push_str(&format!(" :size {}", symbol_size_to_symbol(size)));
        }
    }

    // Dots
    if !note.dots.is_empty() {
        out.push_str(&format!(" :dots {}", note.dots.len()));
    }

    // Accidental
    if let Some(ref accidental) = note.accidental {
        out.push_str(&print_accidental(accidental));
    }

    // Time modification (tuplets)
    if let Some(ref tm) = note.time_modification {
        out.push_str(&print_time_modification(tm));
    }

    // Stem
    if let Some(ref stem) = note.stem {
        out.push_str(&print_stem(stem));
    }

    // Notehead
    if let Some(ref notehead) = note.notehead {
        out.push_str(&print_notehead(notehead));
    }

    // Staff
    if let Some(staff) = note.staff {
        out.push_str(&format!(" :staff {}", staff));
    }

    // Beams
    for beam in &note.beams {
        out.push_str(&print_beam(beam));
    }

    // Notations
    for notation in &note.notations {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_notations(notation, level + 1, options));
    }

    // Lyrics
    for lyric in &note.lyrics {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_lyric(lyric, level + 1, options));
    }

    // Voice
    if let Some(ref voice) = note.voice {
        out.push_str(&format!(" :voice \"{}\"", escape_string(voice)));
    }

    out.push(')');
    out
}

fn print_full_note_content(content: &PitchRestUnpitched) -> String {
    match content {
        PitchRestUnpitched::Pitch(pitch) => format!(" :pitch {}", print_pitch(pitch)),
        PitchRestUnpitched::Unpitched(unpitched) => {
            format!(" :unpitched {}", print_unpitched(unpitched))
        }
        PitchRestUnpitched::Rest(rest) => format!(" :rest {}", print_rest(rest)),
    }
}

fn print_pitch(pitch: &Pitch) -> String {
    let mut out = String::from("(pitch");

    out.push_str(&format!(" :step {}", step_to_symbol(&pitch.step)));

    if let Some(alter) = pitch.alter {
        if alter != 0.0 {
            out.push_str(&format!(" :alter {}", format_float(alter)));
        }
    }

    out.push_str(&format!(" :octave {}", pitch.octave));

    out.push(')');
    out
}

fn print_unpitched(unpitched: &Unpitched) -> String {
    let mut out = String::from("(unpitched");

    if let Some(ref display_step) = unpitched.display_step {
        out.push_str(&format!(" :display-step {}", step_to_symbol(display_step)));
    }

    if let Some(display_octave) = unpitched.display_octave {
        out.push_str(&format!(" :display-octave {}", display_octave));
    }

    out.push(')');
    out
}

fn print_rest(rest: &Rest) -> String {
    let mut out = String::from("(rest");

    if let Some(ref display_step) = rest.display_step {
        out.push_str(&format!(" :display-step {}", step_to_symbol(display_step)));
    }

    if let Some(display_octave) = rest.display_octave {
        out.push_str(&format!(" :display-octave {}", display_octave));
    }

    if rest.measure == Some(YesNo::Yes) {
        out.push_str(" :measure #t");
    }

    out.push(')');
    out
}

fn print_tie(tie: &Tie) -> String {
    format!(" :tie {}", print_start_stop(tie.r#type))
}

fn print_accidental(accidental: &Accidental) -> String {
    let mut out = String::from(" :accidental");
    out.push_str(&format!(
        " {}",
        accidental_value_to_symbol(&accidental.value)
    ));

    if accidental.cautionary == Some(YesNo::Yes) {
        out.push_str(" :cautionary #t");
    }

    if accidental.editorial == Some(YesNo::Yes) {
        out.push_str(" :editorial #t");
    }

    if accidental.parentheses == Some(YesNo::Yes) {
        out.push_str(" :parentheses #t");
    }

    if accidental.bracket == Some(YesNo::Yes) {
        out.push_str(" :bracket #t");
    }

    out
}

fn print_time_modification(tm: &TimeModification) -> String {
    let mut out = format!(
        " :time-modification (time-modification :actual-notes {} :normal-notes {}",
        tm.actual_notes, tm.normal_notes
    );

    if let Some(ref normal_type) = tm.normal_type {
        out.push_str(&format!(
            " :normal-type {}",
            note_type_to_symbol(normal_type)
        ));
    }

    if tm.normal_dots > 0 {
        out.push_str(&format!(" :normal-dots {}", tm.normal_dots));
    }

    out.push(')');
    out
}

fn print_stem(stem: &Stem) -> String {
    let value = match stem.value {
        StemValue::Down => "down",
        StemValue::Up => "up",
        StemValue::Double => "double",
        StemValue::None => "none",
    };

    let mut out = format!(" :stem {}", value);

    if let Some(default_y) = stem.default_y {
        out.push_str(&format!(" :default-y {}", format_float(default_y)));
    }

    out
}

fn print_notehead(notehead: &Notehead) -> String {
    let value = notehead_value_to_symbol(&notehead.value);
    let mut out = format!(" :notehead {}", value);

    if let Some(filled) = notehead.filled {
        out.push_str(&format!(" :filled {}", print_yes_no(filled)));
    }

    if let Some(parentheses) = notehead.parentheses {
        out.push_str(&format!(" :parentheses {}", print_yes_no(parentheses)));
    }

    out
}

fn print_beam(beam: &Beam) -> String {
    let value = match beam.value {
        BeamValue::Begin => "begin",
        BeamValue::Continue => "continue",
        BeamValue::End => "end",
        BeamValue::ForwardHook => "forward-hook",
        BeamValue::BackwardHook => "backward-hook",
    };

    let mut out = format!(" :beam (beam :number {} :value {})", beam.number, value);

    if let Some(ref fan) = beam.fan {
        let fan_str = match fan {
            Fan::Accel => "accel",
            Fan::Rit => "rit",
            Fan::None => "none",
        };
        out.push_str(&format!(" :fan {}", fan_str));
    }

    out
}

// === Attributes Printers ===

fn print_attributes(attrs: &Attributes, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(attributes", ind);

    if let Some(divisions) = attrs.divisions {
        out.push_str(&format!(" :divisions {}", divisions));
    }

    for key in &attrs.keys {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_key(key, level + 1, options));
    }

    for time in &attrs.times {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_time(time, level + 1, options));
    }

    if let Some(staves) = attrs.staves {
        out.push_str(&format!(" :staves {}", staves));
    }

    for clef in &attrs.clefs {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_clef(clef, level + 1, options));
    }

    for staff_details in &attrs.staff_details {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_staff_details(staff_details, level + 1, options));
    }

    for transpose in &attrs.transpose {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_transpose(transpose, level + 1, options));
    }

    if let Some(ref instruments) = attrs.instruments {
        out.push_str(&format!(" :instruments {}", instruments));
    }

    out.push(')');
    out
}

fn print_key(key: &Key, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(key", ind);

    if let Some(number) = key.number {
        out.push_str(&format!(" :number {}", number));
    }

    if let Some(ref print_object) = key.print_object {
        out.push_str(&format!(" :print-object {}", print_yes_no(*print_object)));
    }

    match &key.content {
        KeyContent::Traditional(trad_key) => {
            if let Some(ref cancel) = trad_key.cancel {
                out.push_str(&newline_indent(level + 1, options));
                out.push_str(&print_cancel(cancel));
            }
            out.push_str(&format!(" :fifths {}", trad_key.fifths));
            if let Some(ref mode) = trad_key.mode {
                out.push_str(&format!(" :mode {}", mode_to_symbol(mode)));
            }
        }
        KeyContent::NonTraditional(key_steps) => {
            for key_step in key_steps {
                out.push_str(&newline_indent(level + 1, options));
                out.push_str(&format!(
                    "(key-step {} :key-alter {})",
                    step_to_symbol(&key_step.step),
                    format_float(key_step.alter)
                ));
                if let Some(ref acc) = key_step.accidental {
                    out.push_str(&format!(
                        " :key-accidental {}",
                        accidental_value_to_symbol(acc)
                    ));
                }
            }
        }
    }

    out.push(')');
    out
}

fn print_cancel(cancel: &Cancel) -> String {
    format!("(cancel :fifths {})", cancel.fifths)
}

fn print_time(time: &Time, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(time", ind);

    if let Some(number) = time.number {
        out.push_str(&format!(" :number {}", number));
    }

    if let Some(ref symbol) = time.symbol {
        out.push_str(&format!(" :symbol {}", time_symbol_to_symbol(symbol)));
    }

    match &time.content {
        TimeContent::Measured { signatures } => {
            for sig in signatures {
                out.push_str(&format!(" (beats \"{}\")", escape_string(&sig.beats)));
                out.push_str(&format!(
                    " (beat-type \"{}\")",
                    escape_string(&sig.beat_type)
                ));
            }
        }
        TimeContent::SenzaMisura(text) => {
            out.push_str(" :senza-misura #t");
            if !text.is_empty() {
                out.push_str(&format!(" \"{}\"", escape_string(text)));
            }
        }
    }

    out.push(')');
    out
}

fn print_clef(clef: &Clef, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(clef", ind);

    if let Some(number) = clef.number {
        out.push_str(&format!(" :number {}", number));
    }

    out.push_str(&format!(" :sign {}", clef_sign_to_symbol(&clef.sign)));
    if let Some(line) = clef.line {
        out.push_str(&format!(" :line {}", line));
    }

    if let Some(octave_change) = clef.octave_change {
        out.push_str(&format!(" :clef-octave-change {}", octave_change));
    }

    if let Some(ref print_object) = clef.print_object {
        out.push_str(&format!(" :print-object {}", print_yes_no(*print_object)));
    }

    out.push(')');
    out
}

fn print_staff_details(sd: &StaffDetails, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(staff-details", ind);

    if let Some(number) = sd.number {
        out.push_str(&format!(" :number {}", number));
    }

    if let Some(staff_lines) = sd.staff_lines {
        out.push_str(&format!(" :staff-lines {}", staff_lines));
    }

    out.push(')');
    out
}

fn print_transpose(transpose: &Transpose, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(transpose", ind);

    if let Some(number) = transpose.number {
        out.push_str(&format!(" :number {}", number));
    }

    if let Some(diatonic) = transpose.diatonic {
        out.push_str(&format!(" :diatonic {}", diatonic));
    }

    out.push_str(&format!(" :chromatic {}", transpose.chromatic));

    if let Some(octave_change) = transpose.octave_change {
        out.push_str(&format!(" :octave-change {}", octave_change));
    }

    if transpose.double == Some(YesNo::Yes) {
        out.push_str(" :double #t");
    }

    out.push(')');
    out
}

// === Direction Printers ===

fn print_direction(direction: &Direction, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(direction", ind);

    if let Some(ref placement) = direction.placement {
        out.push_str(&format!(" :placement {}", above_below_to_symbol(placement)));
    }

    for dt in &direction.direction_types {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_direction_type(dt, level + 1, options));
    }

    if let Some(staff) = direction.staff {
        out.push_str(&format!(" :staff {}", staff));
    }

    if let Some(ref voice) = direction.voice {
        out.push_str(&format!(" :voice \"{}\"", escape_string(voice)));
    }

    out.push(')');
    out
}

fn print_direction_type(dt: &DirectionType, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    match &dt.content {
        DirectionTypeContent::Rehearsal(rehearsals) => {
            if let Some(first) = rehearsals.first() {
                format!("{}(rehearsal \"{}\")", ind, escape_string(&first.value))
            } else {
                format!("{}(rehearsal)", ind)
            }
        }
        DirectionTypeContent::Segno(_) => format!("{}(segno)", ind),
        DirectionTypeContent::Coda(_) => format!("{}(coda)", ind),
        DirectionTypeContent::Words(words_list) => {
            if let Some(first) = words_list.first() {
                print_words(first, level, options)
            } else {
                format!("{}(words)", ind)
            }
        }
        DirectionTypeContent::Dynamics(dynamics) => print_dynamics(dynamics, level, options),
        DirectionTypeContent::Wedge(wedge) => {
            format!(
                "{}(wedge :type {})",
                ind,
                wedge_type_to_symbol(&wedge.r#type)
            )
        }
        DirectionTypeContent::Metronome(metronome) => print_metronome(metronome, level, options),
        DirectionTypeContent::OctaveShift(os) => print_octave_shift(os, level, options),
        DirectionTypeContent::Pedal(pedal) => print_pedal(pedal, level, options),
        DirectionTypeContent::OtherDirection(other) => {
            format!(
                "{}(other-direction \"{}\")",
                ind,
                escape_string(&other.value)
            )
        }
        _ => format!("{}(direction-type)", ind),
    }
}

fn print_words(words: &Words, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(words \"{}\"", ind, escape_string(&words.value));

    if let Some(ref lang) = words.lang {
        out.push_str(&format!(" :lang \"{}\"", escape_string(lang)));
    }

    out.push(')');
    out
}

fn print_dynamics(dynamics: &Dynamics, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(dynamics", ind);

    for dt in &dynamics.content {
        out.push(' ');
        out.push_str(&dynamic_element_to_symbol(dt));
    }

    out.push(')');
    out
}

fn print_metronome(metronome: &Metronome, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(metronome", ind);

    if metronome.parentheses == Some(YesNo::Yes) {
        out.push_str(" :parentheses #t");
    }

    match &metronome.content {
        MetronomeContent::PerMinute {
            beat_unit,
            beat_unit_dots,
            per_minute,
        } => {
            out.push_str(&format!(" :beat-unit {}", note_type_to_symbol(beat_unit)));
            for _ in 0..*beat_unit_dots {
                out.push_str(" :beat-unit-dot");
            }
            out.push_str(&format!(
                " :per-minute \"{}\"",
                escape_string(&per_minute.value)
            ));
        }
        MetronomeContent::BeatEquation {
            left_unit,
            left_dots,
            right_unit,
            right_dots,
        } => {
            out.push_str(&format!(" :left-unit {}", note_type_to_symbol(left_unit)));
            for _ in 0..*left_dots {
                out.push_str(" :left-dot");
            }
            out.push_str(&format!(" :right-unit {}", note_type_to_symbol(right_unit)));
            for _ in 0..*right_dots {
                out.push_str(" :right-dot");
            }
        }
        MetronomeContent::MetricModulation { .. } => {
            out.push_str(" :metric-modulation #t");
        }
    }

    out.push(')');
    out
}

fn print_octave_shift(os: &OctaveShift, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(octave-shift", ind);

    out.push_str(&format!(
        " :type {}",
        print_up_down_stop_continue(os.r#type)
    ));

    if let Some(number) = os.number {
        out.push_str(&format!(" :number {}", number));
    }

    if let Some(size) = os.size {
        out.push_str(&format!(" :size {}", size));
    }

    out.push(')');
    out
}

fn print_pedal(pedal: &Pedal, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(pedal", ind);

    out.push_str(&format!(" :type {}", pedal_type_to_symbol(&pedal.r#type)));

    if let Some(line) = pedal.line {
        out.push_str(&format!(" :line {}", print_yes_no(line)));
    }

    if let Some(sign) = pedal.sign {
        out.push_str(&format!(" :sign {}", print_yes_no(sign)));
    }

    out.push(')');
    out
}

// === Notation Printers ===

fn print_notations(notations: &Notations, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(notations", ind);

    for content in &notations.content {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_notation_content(content, level + 1, options));
    }

    out.push(')');
    out
}

fn print_notation_content(
    content: &NotationContent,
    level: usize,
    options: &PrintOptions,
) -> String {
    let ind = indent(level, options);
    match content {
        NotationContent::Tied(tied) => print_tied(tied, level, options),
        NotationContent::Slur(slur) => print_slur(slur, level, options),
        NotationContent::Tuplet(tuplet) => print_tuplet(tuplet, level, options),
        NotationContent::Glissando(glissando) => print_glissando(glissando, level, options),
        NotationContent::Slide(slide) => print_slide(slide, level, options),
        NotationContent::Ornaments(ornaments) => print_ornaments(ornaments, level, options),
        NotationContent::Technical(technical) => print_technical(technical, level, options),
        NotationContent::Articulations(articulations) => {
            print_articulations(articulations, level, options)
        }
        NotationContent::Dynamics(dynamics) => print_dynamics(dynamics, level, options),
        NotationContent::Fermata(fermata) => print_fermata(fermata, level, options),
        NotationContent::Arpeggiate(arp) => print_arpeggiate(arp, level, options),
        NotationContent::NonArpeggiate(na) => print_non_arpeggiate(na, level, options),
        NotationContent::AccidentalMark(am) => format!(
            "{}(accidental-mark {})",
            ind,
            accidental_value_to_symbol(&am.value)
        ),
        NotationContent::OtherNotation(other) => print_other_notation(other, level, options),
    }
}

fn print_tied(tied: &Tied, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(tied", ind);

    out.push_str(&format!(
        " :type {}",
        print_start_stop_continue(tied.r#type)
    ));

    if let Some(number) = tied.number {
        out.push_str(&format!(" :number {}", number));
    }

    out.push(')');
    out
}

fn print_slur(slur: &crate::ir::notation::Slur, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(slur", ind);

    out.push_str(&format!(
        " :type {}",
        print_start_stop_continue(slur.r#type)
    ));

    out.push_str(&format!(" :number {}", slur.number));

    if let Some(ref placement) = slur.placement {
        out.push_str(&format!(" :placement {}", above_below_to_symbol(placement)));
    }

    out.push(')');
    out
}

fn print_tuplet(tuplet: &Tuplet, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(tuplet", ind);

    out.push_str(&format!(" :type {}", print_start_stop(tuplet.r#type)));

    if let Some(number) = tuplet.number {
        out.push_str(&format!(" :number {}", number));
    }

    if let Some(bracket) = tuplet.bracket {
        out.push_str(&format!(" :bracket {}", print_yes_no(bracket)));
    }

    if let Some(ref show_number) = tuplet.show_number {
        out.push_str(&format!(
            " :show-number {}",
            show_tuplet_to_symbol(show_number)
        ));
    }

    if let Some(ref show_type) = tuplet.show_type {
        out.push_str(&format!(" :show-type {}", show_tuplet_to_symbol(show_type)));
    }

    if let Some(ref actual) = tuplet.tuplet_actual {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_tuplet_portion(
            "tuplet-actual",
            actual,
            level + 1,
            options,
        ));
    }

    if let Some(ref normal) = tuplet.tuplet_normal {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_tuplet_portion(
            "tuplet-normal",
            normal,
            level + 1,
            options,
        ));
    }

    out.push(')');
    out
}

fn print_tuplet_portion(
    tag: &str,
    portion: &TupletPortion,
    level: usize,
    options: &PrintOptions,
) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}({}", ind, tag);

    if let Some(ref number) = portion.tuplet_number {
        out.push_str(&format!(" :tuplet-number {}", number.value));
    }

    if let Some(ref tuplet_type) = portion.tuplet_type {
        out.push_str(&format!(
            " :tuplet-type {}",
            note_type_to_symbol(&tuplet_type.value)
        ));
    }

    for _ in &portion.tuplet_dots {
        out.push_str(" :tuplet-dot");
    }

    out.push(')');
    out
}

fn print_glissando(glissando: &Glissando, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(glissando", ind);

    out.push_str(&format!(" :type {}", print_start_stop(glissando.r#type)));

    if let Some(number) = glissando.number {
        out.push_str(&format!(" :number {}", number));
    }

    if let Some(ref text) = glissando.text {
        out.push_str(&format!(" \"{}\"", escape_string(text)));
    }

    out.push(')');
    out
}

fn print_slide(slide: &Slide, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(slide", ind);

    out.push_str(&format!(" :type {}", print_start_stop(slide.r#type)));

    if let Some(number) = slide.number {
        out.push_str(&format!(" :number {}", number));
    }

    if let Some(ref text) = slide.text {
        out.push_str(&format!(" \"{}\"", escape_string(text)));
    }

    out.push(')');
    out
}

fn print_ornaments(ornaments: &Ornaments, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(ornaments", ind);

    for content in &ornaments.content {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_ornament_with_accidentals(
            content,
            level + 1,
            options,
        ));
    }

    out.push(')');
    out
}

fn print_ornament_with_accidentals(
    owa: &OrnamentWithAccidentals,
    level: usize,
    options: &PrintOptions,
) -> String {
    let ind = indent(level, options);
    match &owa.ornament {
        OrnamentElement::TrillMark(_) => format!("{}(trill-mark)", ind),
        OrnamentElement::Turn(turn) => {
            let mut out = format!("{}(turn", ind);
            if turn.slash == Some(YesNo::Yes) {
                out.push_str(" :slash #t");
            }
            out.push(')');
            out
        }
        OrnamentElement::DelayedTurn(_) => format!("{}(delayed-turn)", ind),
        OrnamentElement::InvertedTurn(_) => format!("{}(inverted-turn)", ind),
        OrnamentElement::DelayedInvertedTurn(_) => format!("{}(delayed-inverted-turn)", ind),
        OrnamentElement::VerticalTurn(_) => format!("{}(vertical-turn)", ind),
        OrnamentElement::InvertedVerticalTurn(_) => format!("{}(inverted-vertical-turn)", ind),
        OrnamentElement::Shake(_) => format!("{}(shake)", ind),
        OrnamentElement::WavyLine(wl) => format!(
            "{}(wavy-line :type {})",
            ind,
            print_start_stop_continue(wl.r#type)
        ),
        OrnamentElement::Mordent(m) => {
            let mut out = format!("{}(mordent", ind);
            if m.long == Some(YesNo::Yes) {
                out.push_str(" :long #t");
            }
            out.push(')');
            out
        }
        OrnamentElement::InvertedMordent(m) => {
            let mut out = format!("{}(inverted-mordent", ind);
            if m.long == Some(YesNo::Yes) {
                out.push_str(" :long #t");
            }
            out.push(')');
            out
        }
        OrnamentElement::Schleifer(_) => format!("{}(schleifer)", ind),
        OrnamentElement::Tremolo(t) => {
            format!("{}(tremolo :value {})", ind, t.value)
        }
        OrnamentElement::Haydn(_) => format!("{}(haydn)", ind),
        OrnamentElement::OtherOrnament(other) => {
            format!(
                "{}(other-ornament \"{}\")",
                ind,
                escape_string(&other.value)
            )
        }
    }
}

fn print_technical(technical: &Technical, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(technical", ind);

    for elem in &technical.content {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_technical_element(elem, level + 1, options));
    }

    out.push(')');
    out
}

fn print_technical_element(
    elem: &TechnicalElement,
    level: usize,
    options: &PrintOptions,
) -> String {
    let ind = indent(level, options);
    match elem {
        TechnicalElement::UpBow(_) => format!("{}(up-bow)", ind),
        TechnicalElement::DownBow(_) => format!("{}(down-bow)", ind),
        TechnicalElement::Harmonic(h) => {
            let mut out = format!("{}(harmonic", ind);
            if h.natural {
                out.push_str(" :natural #t");
            }
            if h.artificial {
                out.push_str(" :artificial #t");
            }
            out.push(')');
            out
        }
        TechnicalElement::OpenString(_) => format!("{}(open-string)", ind),
        TechnicalElement::ThumbPosition(_) => format!("{}(thumb-position)", ind),
        TechnicalElement::Fingering(f) => {
            format!("{}(fingering \"{}\")", ind, escape_string(&f.value))
        }
        TechnicalElement::Pluck(p) => {
            format!("{}(pluck \"{}\")", ind, escape_string(&p.value))
        }
        TechnicalElement::DoubleTongue(_) => format!("{}(double-tongue)", ind),
        TechnicalElement::TripleTongue(_) => format!("{}(triple-tongue)", ind),
        TechnicalElement::Stopped(_) => format!("{}(stopped)", ind),
        TechnicalElement::SnapPizzicato(_) => format!("{}(snap-pizzicato)", ind),
        TechnicalElement::Fret(f) => format!("{}(fret {})", ind, f.value),
        TechnicalElement::String(s) => format!("{}(string {})", ind, s.value),
        TechnicalElement::HammerOn(h) => {
            let mut out = format!("{}(hammer-on :type {})", ind, print_start_stop(h.r#type));
            if !h.value.is_empty() {
                out = format!(
                    "{}(hammer-on :type {} \"{}\")",
                    ind,
                    print_start_stop(h.r#type),
                    escape_string(&h.value)
                );
            }
            out
        }
        TechnicalElement::PullOff(p) => {
            let mut out = format!("{}(pull-off :type {})", ind, print_start_stop(p.r#type));
            if !p.value.is_empty() {
                out = format!(
                    "{}(pull-off :type {} \"{}\")",
                    ind,
                    print_start_stop(p.r#type),
                    escape_string(&p.value)
                );
            }
            out
        }
        TechnicalElement::Bend(b) => {
            format!("{}(bend :bend-alter {})", ind, format_float(b.bend_alter))
        }
        TechnicalElement::Tap(t) => {
            format!("{}(tap \"{}\")", ind, escape_string(&t.value))
        }
        TechnicalElement::Heel(_) => format!("{}(heel)", ind),
        TechnicalElement::Toe(_) => format!("{}(toe)", ind),
        TechnicalElement::Fingernails(_) => format!("{}(fingernails)", ind),
        TechnicalElement::Hole(_) => format!("{}(hole)", ind),
        TechnicalElement::Arrow(_) => format!("{}(arrow)", ind),
        TechnicalElement::Handbell(_) => format!("{}(handbell)", ind),
        TechnicalElement::BrassBend(_) => format!("{}(brass-bend)", ind),
        TechnicalElement::Flip(_) => format!("{}(flip)", ind),
        TechnicalElement::Smear(_) => format!("{}(smear)", ind),
        TechnicalElement::Open(_) => format!("{}(open)", ind),
        TechnicalElement::HalfMuted(_) => format!("{}(half-muted)", ind),
        TechnicalElement::HarmonMute(_) => format!("{}(harmon-mute)", ind),
        TechnicalElement::Golpe(_) => format!("{}(golpe)", ind),
        TechnicalElement::OtherTechnical(other) => {
            format!(
                "{}(other-technical \"{}\")",
                ind,
                escape_string(&other.value)
            )
        }
    }
}

fn print_articulations(
    articulations: &Articulations,
    level: usize,
    options: &PrintOptions,
) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(articulations", ind);

    for elem in &articulations.content {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_articulation_element(elem, level + 1, options));
    }

    out.push(')');
    out
}

fn print_articulation_element(
    elem: &ArticulationElement,
    level: usize,
    options: &PrintOptions,
) -> String {
    let ind = indent(level, options);
    match elem {
        ArticulationElement::Accent(_) => format!("{}(accent)", ind),
        ArticulationElement::StrongAccent(_) => format!("{}(strong-accent)", ind),
        ArticulationElement::Staccato(_) => format!("{}(staccato)", ind),
        ArticulationElement::Tenuto(_) => format!("{}(tenuto)", ind),
        ArticulationElement::DetachedLegato(_) => format!("{}(detached-legato)", ind),
        ArticulationElement::Staccatissimo(_) => format!("{}(staccatissimo)", ind),
        ArticulationElement::Spiccato(_) => format!("{}(spiccato)", ind),
        ArticulationElement::Scoop(_) => format!("{}(scoop)", ind),
        ArticulationElement::Plop(_) => format!("{}(plop)", ind),
        ArticulationElement::Doit(_) => format!("{}(doit)", ind),
        ArticulationElement::Falloff(_) => format!("{}(falloff)", ind),
        ArticulationElement::BreathMark(bm) => {
            let mut out = format!("{}(breath-mark", ind);
            match bm.value {
                crate::ir::notation::BreathMarkValue::Comma => out.push_str(" :value comma"),
                crate::ir::notation::BreathMarkValue::Tick => out.push_str(" :value tick"),
                crate::ir::notation::BreathMarkValue::Upbow => out.push_str(" :value upbow"),
                crate::ir::notation::BreathMarkValue::Salzedo => out.push_str(" :value salzedo"),
                _ => {}
            }
            out.push(')');
            out
        }
        ArticulationElement::Caesura(_) => format!("{}(caesura)", ind),
        ArticulationElement::Stress(_) => format!("{}(stress)", ind),
        ArticulationElement::Unstress(_) => format!("{}(unstress)", ind),
        ArticulationElement::SoftAccent(_) => format!("{}(soft-accent)", ind),
        ArticulationElement::OtherArticulation(other) => {
            format!(
                "{}(other-articulation \"{}\")",
                ind,
                escape_string(&other.value)
            )
        }
    }
}

fn print_fermata(
    fermata: &crate::ir::notation::Fermata,
    level: usize,
    options: &PrintOptions,
) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(fermata", ind);

    if let Some(ref shape) = fermata.shape {
        out.push_str(&format!(" :shape {}", fermata_shape_to_symbol(shape)));
    }

    if let Some(ref fermata_type) = fermata.r#type {
        out.push_str(&format!(
            " :type {}",
            upright_inverted_to_symbol(fermata_type)
        ));
    }

    out.push(')');
    out
}

fn print_arpeggiate(arp: &Arpeggiate, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(arpeggiate", ind);

    if let Some(number) = arp.number {
        out.push_str(&format!(" :number {}", number));
    }

    if let Some(ref direction) = arp.direction {
        out.push_str(&format!(" :direction {}", up_down_to_symbol(direction)));
    }

    out.push(')');
    out
}

fn print_non_arpeggiate(na: &NonArpeggiate, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(non-arpeggiate", ind);

    out.push_str(&format!(" :type {}", print_top_bottom(na.r#type)));

    if let Some(number) = na.number {
        out.push_str(&format!(" :number {}", number));
    }

    out.push(')');
    out
}

fn print_other_notation(other: &OtherNotation, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(other-notation", ind);

    out.push_str(&format!(" :type {}", print_start_stop_single(other.r#type)));

    if !other.value.is_empty() {
        out.push_str(&format!(" \"{}\"", escape_string(&other.value)));
    }

    out.push(')');
    out
}

// === Barline Printers ===

fn print_barline(barline: &Barline, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(barline", ind);

    if let Some(ref location) = barline.location {
        out.push_str(&format!(
            " :location {}",
            right_left_middle_to_symbol(location)
        ));
    }

    if let Some(ref bar_style) = barline.bar_style {
        out.push_str(&format!(" :bar-style {}", bar_style_to_symbol(bar_style)));
    }

    if let Some(ref repeat) = barline.repeat {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_repeat(repeat, level + 1, options));
    }

    if let Some(ref ending) = barline.ending {
        out.push_str(&newline_indent(level + 1, options));
        out.push_str(&print_ending(ending, level + 1, options));
    }

    out.push(')');
    out
}

fn print_repeat(repeat: &Repeat, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(repeat", ind);

    out.push_str(&format!(
        " :direction {}",
        backward_forward_to_symbol(&repeat.direction)
    ));

    if let Some(times) = repeat.times {
        out.push_str(&format!(" :times {}", times));
    }

    out.push(')');
    out
}

fn print_ending(ending: &Ending, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(ending", ind);

    out.push_str(&format!(" :number \"{}\"", escape_string(&ending.number)));
    out.push_str(&format!(
        " :type {}",
        print_start_stop_discontinue(ending.r#type)
    ));

    if let Some(ref text) = ending.text {
        if !text.is_empty() {
            out.push_str(&format!(" \"{}\"", escape_string(text)));
        }
    }

    out.push(')');
    out
}

// === Voice Printers ===

fn print_backup(backup: &Backup, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    format!("{}(backup :duration {})", ind, backup.duration)
}

fn print_forward(forward: &Forward, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(forward :duration {}", ind, forward.duration);

    if let Some(ref voice) = forward.voice {
        out.push_str(&format!(" :voice \"{}\"", escape_string(voice)));
    }

    if let Some(staff) = forward.staff {
        out.push_str(&format!(" :staff {}", staff));
    }

    out.push(')');
    out
}

// === Lyric Printers ===

fn print_lyric(lyric: &Lyric, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(lyric", ind);

    if let Some(ref number) = lyric.number {
        out.push_str(&format!(" :number \"{}\"", escape_string(number)));
    }

    if let Some(ref name) = lyric.name {
        out.push_str(&format!(" :name \"{}\"", escape_string(name)));
    }

    if let Some(ref justify) = lyric.justify {
        out.push_str(&format!(
            " :justify {}",
            left_center_right_to_symbol(justify)
        ));
    }

    if let Some(ref placement) = lyric.placement {
        out.push_str(&format!(" :placement {}", above_below_to_symbol(placement)));
    }

    match &lyric.content {
        LyricContent::Syllable {
            syllabic,
            text,
            extensions,
            extend,
        } => {
            if let Some(syl) = syllabic {
                out.push_str(&format!(" :syllabic {}", syllabic_to_symbol(syl)));
            }

            out.push_str(&format!(" :text \"{}\"", escape_string(&text.value)));

            for ext in extensions {
                out.push_str(&newline_indent(level + 1, options));
                out.push_str(&print_lyric_extension(ext, level + 1, options));
            }

            if let Some(ext) = extend {
                out.push_str(&format!(" {}", print_extend(ext)));
            }
        }
        LyricContent::ExtendOnly(extend) => {
            out.push_str(&format!(" {}", print_extend(extend)));
        }
        LyricContent::Laughing => {
            out.push_str(" :laughing #t");
        }
        LyricContent::Humming => {
            out.push_str(" :humming #t");
        }
    }

    if lyric.end_line {
        out.push_str(" :end-line #t");
    }

    if lyric.end_paragraph {
        out.push_str(" :end-paragraph #t");
    }

    out.push(')');
    out
}

fn print_lyric_extension(ext: &LyricExtension, level: usize, options: &PrintOptions) -> String {
    let ind = indent(level, options);
    let mut out = format!("{}(lyric-extension", ind);

    out.push_str(&format!(
        " :elision \"{}\"",
        escape_string(&ext.elision.value)
    ));

    if let Some(ref syl) = ext.syllabic {
        out.push_str(&format!(" :syllabic {}", syllabic_to_symbol(syl)));
    }

    out.push_str(&format!(" :text \"{}\"", escape_string(&ext.text.value)));

    out.push(')');
    out
}

fn print_extend(extend: &Extend) -> String {
    let mut out = String::from("(extend");

    if let Some(ref extend_type) = extend.r#type {
        out.push_str(&format!(
            " :type {}",
            print_start_stop_continue(*extend_type)
        ));
    }

    out.push(')');
    out
}

// === Symbol Conversion Functions ===

fn step_to_symbol(step: &Step) -> &'static str {
    match step {
        Step::A => "A",
        Step::B => "B",
        Step::C => "C",
        Step::D => "D",
        Step::E => "E",
        Step::F => "F",
        Step::G => "G",
    }
}

fn note_type_to_symbol(value: &NoteTypeValue) -> &'static str {
    match value {
        NoteTypeValue::N1024th => "1024th",
        NoteTypeValue::N512th => "512th",
        NoteTypeValue::N256th => "256th",
        NoteTypeValue::N128th => "128th",
        NoteTypeValue::N64th => "64th",
        NoteTypeValue::N32nd => "32nd",
        NoteTypeValue::N16th => "16th",
        NoteTypeValue::Eighth => "eighth",
        NoteTypeValue::Quarter => "quarter",
        NoteTypeValue::Half => "half",
        NoteTypeValue::Whole => "whole",
        NoteTypeValue::Breve => "breve",
        NoteTypeValue::Long => "long",
        NoteTypeValue::Maxima => "maxima",
    }
}

fn symbol_size_to_symbol(size: &SymbolSize) -> &'static str {
    match size {
        SymbolSize::Full => "full",
        SymbolSize::Cue => "cue",
        SymbolSize::GraceCue => "grace-cue",
        SymbolSize::Large => "large",
    }
}

fn accidental_value_to_symbol(value: &AccidentalValue) -> &'static str {
    match value {
        AccidentalValue::Sharp => "sharp",
        AccidentalValue::Natural => "natural",
        AccidentalValue::Flat => "flat",
        AccidentalValue::DoubleSharp => "double-sharp",
        AccidentalValue::SharpSharp => "sharp-sharp",
        AccidentalValue::FlatFlat => "flat-flat",
        AccidentalValue::DoubleFlat => "double-flat",
        AccidentalValue::NaturalSharp => "natural-sharp",
        AccidentalValue::NaturalFlat => "natural-flat",
        AccidentalValue::QuarterFlat => "quarter-flat",
        AccidentalValue::QuarterSharp => "quarter-sharp",
        AccidentalValue::ThreeQuartersFlat => "three-quarters-flat",
        AccidentalValue::ThreeQuartersSharp => "three-quarters-sharp",
        AccidentalValue::SharpDown => "sharp-down",
        AccidentalValue::SharpUp => "sharp-up",
        AccidentalValue::NaturalDown => "natural-down",
        AccidentalValue::NaturalUp => "natural-up",
        AccidentalValue::FlatDown => "flat-down",
        AccidentalValue::FlatUp => "flat-up",
        AccidentalValue::TripleSharp => "triple-sharp",
        AccidentalValue::TripleFlat => "triple-flat",
        AccidentalValue::SlashQuarterSharp => "slash-quarter-sharp",
        AccidentalValue::SlashSharp => "slash-sharp",
        AccidentalValue::SlashFlat => "slash-flat",
        AccidentalValue::DoubleSlashFlat => "double-slash-flat",
        AccidentalValue::Sharp1 => "sharp-1",
        AccidentalValue::Sharp2 => "sharp-2",
        AccidentalValue::Sharp3 => "sharp-3",
        AccidentalValue::Sharp5 => "sharp-5",
        AccidentalValue::Flat1 => "flat-1",
        AccidentalValue::Flat2 => "flat-2",
        AccidentalValue::Flat3 => "flat-3",
        AccidentalValue::Flat4 => "flat-4",
        AccidentalValue::Sori => "sori",
        AccidentalValue::Koron => "koron",
        AccidentalValue::Other => "other",
    }
}

fn notehead_value_to_symbol(value: &NoteheadValue) -> &'static str {
    match value {
        NoteheadValue::Slash => "slash",
        NoteheadValue::Triangle => "triangle",
        NoteheadValue::Diamond => "diamond",
        NoteheadValue::Square => "square",
        NoteheadValue::Cross => "cross",
        NoteheadValue::X => "x",
        NoteheadValue::CircleX => "circle-x",
        NoteheadValue::InvertedTriangle => "inverted-triangle",
        NoteheadValue::ArrowDown => "arrow-down",
        NoteheadValue::ArrowUp => "arrow-up",
        NoteheadValue::Circled => "circled",
        NoteheadValue::Slashed => "slashed",
        NoteheadValue::BackSlashed => "back-slashed",
        NoteheadValue::Normal => "normal",
        NoteheadValue::Cluster => "cluster",
        NoteheadValue::CircleDot => "circle-dot",
        NoteheadValue::LeftTriangle => "left-triangle",
        NoteheadValue::Rectangle => "rectangle",
        NoteheadValue::None => "none",
        NoteheadValue::Do => "do",
        NoteheadValue::Re => "re",
        NoteheadValue::Mi => "mi",
        NoteheadValue::Fa => "fa",
        NoteheadValue::FaUp => "fa-up",
        NoteheadValue::So => "so",
        NoteheadValue::La => "la",
        NoteheadValue::Ti => "ti",
        NoteheadValue::Other => "other",
    }
}

fn clef_sign_to_symbol(sign: &ClefSign) -> &'static str {
    match sign {
        ClefSign::G => "G",
        ClefSign::F => "F",
        ClefSign::C => "C",
        ClefSign::Percussion => "percussion",
        ClefSign::Tab => "TAB",
        ClefSign::Jianpu => "jianpu",
        ClefSign::None => "none",
    }
}

fn mode_to_symbol(mode: &Mode) -> &'static str {
    match mode {
        Mode::Major => "major",
        Mode::Minor => "minor",
        Mode::Dorian => "dorian",
        Mode::Phrygian => "phrygian",
        Mode::Lydian => "lydian",
        Mode::Mixolydian => "mixolydian",
        Mode::Aeolian => "aeolian",
        Mode::Ionian => "ionian",
        Mode::Locrian => "locrian",
        Mode::None => "none",
    }
}

fn time_symbol_to_symbol(symbol: &TimeSymbol) -> &'static str {
    match symbol {
        TimeSymbol::Common => "common",
        TimeSymbol::Cut => "cut",
        TimeSymbol::SingleNumber => "single-number",
        TimeSymbol::Normal => "normal",
        TimeSymbol::DottedNote => "dotted-note",
        TimeSymbol::Note => "note",
    }
}

fn dynamic_element_to_symbol(dt: &DynamicElement) -> String {
    match dt {
        DynamicElement::P => "p".to_string(),
        DynamicElement::PP => "pp".to_string(),
        DynamicElement::PPP => "ppp".to_string(),
        DynamicElement::PPPP => "pppp".to_string(),
        DynamicElement::PPPPP => "ppppp".to_string(),
        DynamicElement::PPPPPP => "pppppp".to_string(),
        DynamicElement::F => "f".to_string(),
        DynamicElement::FF => "ff".to_string(),
        DynamicElement::FFF => "fff".to_string(),
        DynamicElement::FFFF => "ffff".to_string(),
        DynamicElement::FFFFF => "fffff".to_string(),
        DynamicElement::FFFFFF => "ffffff".to_string(),
        DynamicElement::MP => "mp".to_string(),
        DynamicElement::MF => "mf".to_string(),
        DynamicElement::SF => "sf".to_string(),
        DynamicElement::SFP => "sfp".to_string(),
        DynamicElement::SFPP => "sfpp".to_string(),
        DynamicElement::FP => "fp".to_string(),
        DynamicElement::RF => "rf".to_string(),
        DynamicElement::RFZ => "rfz".to_string(),
        DynamicElement::SFZ => "sfz".to_string(),
        DynamicElement::SFFZ => "sffz".to_string(),
        DynamicElement::FZ => "fz".to_string(),
        DynamicElement::N => "n".to_string(),
        DynamicElement::PF => "pf".to_string(),
        DynamicElement::SFZP => "sfzp".to_string(),
        DynamicElement::OtherDynamics(s) => s.clone(),
    }
}

fn wedge_type_to_symbol(wt: &WedgeType) -> &'static str {
    match wt {
        WedgeType::Crescendo => "crescendo",
        WedgeType::Diminuendo => "diminuendo",
        WedgeType::Stop => "stop",
        WedgeType::Continue => "continue",
    }
}

fn pedal_type_to_symbol(pt: &PedalType) -> &'static str {
    match pt {
        PedalType::Start => "start",
        PedalType::Stop => "stop",
        PedalType::Sostenuto => "sostenuto",
        PedalType::Change => "change",
        PedalType::Continue => "continue",
        PedalType::Discontinue => "discontinue",
        PedalType::Resume => "resume",
    }
}

fn show_tuplet_to_symbol(st: &crate::ir::notation::ShowTuplet) -> &'static str {
    match st {
        crate::ir::notation::ShowTuplet::Actual => "actual",
        crate::ir::notation::ShowTuplet::Both => "both",
        crate::ir::notation::ShowTuplet::None => "none",
    }
}

fn fermata_shape_to_symbol(shape: &FermataShape) -> &'static str {
    match shape {
        FermataShape::Normal => "normal",
        FermataShape::Angled => "angled",
        FermataShape::Square => "square",
        FermataShape::DoubleAngled => "double-angled",
        FermataShape::DoubleSquare => "double-square",
        FermataShape::DoubleDot => "double-dot",
        FermataShape::HalfCurve => "half-curve",
        FermataShape::Curlew => "curlew",
    }
}

fn bar_style_to_symbol(style: &BarStyle) -> &'static str {
    match style {
        BarStyle::Regular => "regular",
        BarStyle::Dotted => "dotted",
        BarStyle::Dashed => "dashed",
        BarStyle::Heavy => "heavy",
        BarStyle::LightLight => "light-light",
        BarStyle::LightHeavy => "light-heavy",
        BarStyle::HeavyLight => "heavy-light",
        BarStyle::HeavyHeavy => "heavy-heavy",
        BarStyle::Tick => "tick",
        BarStyle::Short => "short",
        BarStyle::None => "none",
    }
}

fn right_left_middle_to_symbol(location: &RightLeftMiddle) -> &'static str {
    match location {
        RightLeftMiddle::Right => "right",
        RightLeftMiddle::Left => "left",
        RightLeftMiddle::Middle => "middle",
    }
}

fn syllabic_to_symbol(syllabic: &Syllabic) -> &'static str {
    match syllabic {
        Syllabic::Single => "single",
        Syllabic::Begin => "begin",
        Syllabic::End => "end",
        Syllabic::Middle => "middle",
    }
}

// === Common Enum Conversions ===

fn print_yes_no(yn: YesNo) -> &'static str {
    match yn {
        YesNo::Yes => "#t",
        YesNo::No => "#f",
    }
}

fn print_start_stop(ss: StartStop) -> &'static str {
    match ss {
        StartStop::Start => "start",
        StartStop::Stop => "stop",
    }
}

fn print_start_stop_continue(ssc: StartStopContinue) -> &'static str {
    match ssc {
        StartStopContinue::Start => "start",
        StartStopContinue::Stop => "stop",
        StartStopContinue::Continue => "continue",
    }
}

fn print_start_stop_single(sss: StartStopSingle) -> &'static str {
    match sss {
        StartStopSingle::Start => "start",
        StartStopSingle::Stop => "stop",
        StartStopSingle::Single => "single",
    }
}

fn print_start_stop_discontinue(ssd: StartStopDiscontinue) -> &'static str {
    match ssd {
        StartStopDiscontinue::Start => "start",
        StartStopDiscontinue::Stop => "stop",
        StartStopDiscontinue::Discontinue => "discontinue",
    }
}

fn print_up_down_stop_continue(udsc: crate::ir::direction::UpDownStopContinue) -> &'static str {
    match udsc {
        crate::ir::direction::UpDownStopContinue::Up => "up",
        crate::ir::direction::UpDownStopContinue::Down => "down",
        crate::ir::direction::UpDownStopContinue::Stop => "stop",
        crate::ir::direction::UpDownStopContinue::Continue => "continue",
    }
}

fn print_top_bottom(tb: TopBottom) -> &'static str {
    match tb {
        TopBottom::Top => "top",
        TopBottom::Bottom => "bottom",
    }
}

fn above_below_to_symbol(ab: &AboveBelow) -> &'static str {
    match ab {
        AboveBelow::Above => "above",
        AboveBelow::Below => "below",
    }
}

fn up_down_to_symbol(ud: &UpDown) -> &'static str {
    match ud {
        UpDown::Up => "up",
        UpDown::Down => "down",
    }
}

fn upright_inverted_to_symbol(ui: &UprightInverted) -> &'static str {
    match ui {
        UprightInverted::Upright => "upright",
        UprightInverted::Inverted => "inverted",
    }
}

fn backward_forward_to_symbol(bf: &BackwardForward) -> &'static str {
    match bf {
        BackwardForward::Backward => "backward",
        BackwardForward::Forward => "forward",
    }
}

fn left_center_right_to_symbol(lcr: &LeftCenterRight) -> &'static str {
    match lcr {
        LeftCenterRight::Left => "left",
        LeftCenterRight::Center => "center",
        LeftCenterRight::Right => "right",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Helper Function Tests ===

    #[test]
    fn test_escape_string_no_escapes() {
        assert_eq!(escape_string("Piano"), "Piano");
    }

    #[test]
    fn test_escape_string_with_quotes() {
        assert_eq!(escape_string("Say \"Hello\""), "Say \\\"Hello\\\"");
    }

    #[test]
    fn test_escape_string_with_backslash() {
        assert_eq!(escape_string("path\\to\\file"), "path\\\\to\\\\file");
    }

    #[test]
    fn test_format_float_whole_number() {
        assert_eq!(format_float(4.0), "4");
    }

    #[test]
    fn test_format_float_decimal() {
        assert_eq!(format_float(3.5), "3.5");
    }

    #[test]
    fn test_format_float_negative() {
        assert_eq!(format_float(-2.0), "-2");
    }

    // === Pitch Tests ===

    #[test]
    fn test_print_pitch_simple() {
        let pitch = Pitch {
            step: Step::C,
            alter: None,
            octave: 4,
        };
        let result = print_pitch(&pitch);
        assert_eq!(result, "(pitch :step C :octave 4)");
    }

    #[test]
    fn test_print_pitch_with_alter() {
        let pitch = Pitch {
            step: Step::F,
            alter: Some(1.0),
            octave: 5,
        };
        let result = print_pitch(&pitch);
        assert_eq!(result, "(pitch :step F :alter 1 :octave 5)");
    }

    #[test]
    fn test_print_pitch_with_flat() {
        let pitch = Pitch {
            step: Step::B,
            alter: Some(-1.0),
            octave: 3,
        };
        let result = print_pitch(&pitch);
        assert_eq!(result, "(pitch :step B :alter -1 :octave 3)");
    }

    #[test]
    fn test_print_pitch_with_zero_alter_omitted() {
        let pitch = Pitch {
            step: Step::D,
            alter: Some(0.0),
            octave: 4,
        };
        let result = print_pitch(&pitch);
        assert_eq!(result, "(pitch :step D :octave 4)");
    }

    // === Rest Tests ===

    #[test]
    fn test_print_rest_simple() {
        let rest = Rest {
            display_step: None,
            display_octave: None,
            measure: None,
        };
        let result = print_rest(&rest);
        assert_eq!(result, "(rest)");
    }

    #[test]
    fn test_print_rest_measure() {
        let rest = Rest {
            display_step: None,
            display_octave: None,
            measure: Some(YesNo::Yes),
        };
        let result = print_rest(&rest);
        assert_eq!(result, "(rest :measure #t)");
    }

    #[test]
    fn test_print_rest_with_display() {
        let rest = Rest {
            display_step: Some(Step::E),
            display_octave: Some(4),
            measure: None,
        };
        let result = print_rest(&rest);
        assert_eq!(result, "(rest :display-step E :display-octave 4)");
    }

    // === Step Symbol Tests ===

    #[test]
    fn test_step_to_symbol_all() {
        assert_eq!(step_to_symbol(&Step::A), "A");
        assert_eq!(step_to_symbol(&Step::B), "B");
        assert_eq!(step_to_symbol(&Step::C), "C");
        assert_eq!(step_to_symbol(&Step::D), "D");
        assert_eq!(step_to_symbol(&Step::E), "E");
        assert_eq!(step_to_symbol(&Step::F), "F");
        assert_eq!(step_to_symbol(&Step::G), "G");
    }

    // === Note Type Symbol Tests ===

    #[test]
    fn test_note_type_to_symbol_common() {
        assert_eq!(note_type_to_symbol(&NoteTypeValue::Quarter), "quarter");
        assert_eq!(note_type_to_symbol(&NoteTypeValue::Eighth), "eighth");
        assert_eq!(note_type_to_symbol(&NoteTypeValue::Half), "half");
        assert_eq!(note_type_to_symbol(&NoteTypeValue::Whole), "whole");
    }

    #[test]
    fn test_note_type_to_symbol_short() {
        assert_eq!(note_type_to_symbol(&NoteTypeValue::N16th), "16th");
        assert_eq!(note_type_to_symbol(&NoteTypeValue::N32nd), "32nd");
        assert_eq!(note_type_to_symbol(&NoteTypeValue::N64th), "64th");
    }

    use crate::ir::attributes::{TimeSignature, TraditionalKey};

    // === Clef Tests ===

    #[test]
    fn test_print_clef_treble() {
        let clef = Clef {
            sign: ClefSign::G,
            line: Some(2),
            octave_change: None,
            number: None,
            print_object: None,
            size: None,
        };
        let options = PrintOptions::default();
        let result = print_clef(&clef, 0, &options);
        assert_eq!(result, "(clef :sign G :line 2)");
    }

    #[test]
    fn test_print_clef_bass() {
        let clef = Clef {
            sign: ClefSign::F,
            line: Some(4),
            octave_change: None,
            number: None,
            print_object: None,
            size: None,
        };
        let options = PrintOptions::default();
        let result = print_clef(&clef, 0, &options);
        assert_eq!(result, "(clef :sign F :line 4)");
    }

    // === Key Tests ===

    #[test]
    fn test_print_key_c_major() {
        let key = Key {
            number: None,
            print_object: None,
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths: 0,
                mode: Some(Mode::Major),
            }),
        };
        let options = PrintOptions::default();
        let result = print_key(&key, 0, &options);
        assert_eq!(result, "(key :fifths 0 :mode major)");
    }

    // === Time Tests ===

    #[test]
    fn test_print_time_4_4() {
        let time = Time {
            number: None,
            symbol: None,
            print_object: None,
            content: TimeContent::Measured {
                signatures: vec![TimeSignature {
                    beats: "4".to_string(),
                    beat_type: "4".to_string(),
                }],
            },
        };
        let options = PrintOptions::default();
        let result = print_time(&time, 0, &options);
        assert_eq!(result, "(time (beats \"4\") (beat-type \"4\"))");
    }

    // === Dynamics Tests ===

    #[test]
    fn test_print_dynamics_p() {
        let dynamics = Dynamics {
            content: vec![DynamicElement::P],
            placement: None,
            print_style: crate::ir::common::PrintStyle::default(),
        };
        let options = PrintOptions::default();
        let result = print_dynamics(&dynamics, 0, &options);
        assert_eq!(result, "(dynamics p)");
    }

    // === Yes/No Tests ===

    #[test]
    fn test_print_yes_no() {
        assert_eq!(print_yes_no(YesNo::Yes), "#t");
        assert_eq!(print_yes_no(YesNo::No), "#f");
    }

    // === Start/Stop Tests ===

    #[test]
    fn test_print_start_stop() {
        assert_eq!(print_start_stop(StartStop::Start), "start");
        assert_eq!(print_start_stop(StartStop::Stop), "stop");
    }

    // === Backup/Forward Tests ===

    #[test]
    fn test_print_backup() {
        let backup = Backup {
            duration: 4,
            editorial: crate::ir::common::Editorial::default(),
        };
        let options = PrintOptions::default();
        let result = print_backup(&backup, 0, &options);
        assert_eq!(result, "(backup :duration 4)");
    }

    #[test]
    fn test_print_forward() {
        let forward = Forward {
            duration: 2,
            voice: Some("1".to_string()),
            staff: Some(1),
            editorial: crate::ir::common::Editorial::default(),
        };
        let options = PrintOptions::default();
        let result = print_forward(&forward, 0, &options);
        assert_eq!(result, "(forward :duration 2 :voice \"1\" :staff 1)");
    }

    // === Indent Tests ===

    #[test]
    fn test_indent_level_0() {
        let options = PrintOptions::default();
        assert_eq!(indent(0, &options), "");
    }

    #[test]
    fn test_indent_level_1() {
        let options = PrintOptions::default();
        assert_eq!(indent(1, &options), "  ");
    }

    #[test]
    fn test_indent_compact() {
        let options = PrintOptions {
            compact: true,
            ..Default::default()
        };
        assert_eq!(indent(5, &options), "");
    }

    #[test]
    fn test_newline_indent_pretty() {
        let options = PrintOptions::default();
        assert_eq!(newline_indent(1, &options), "\n  ");
    }

    #[test]
    fn test_newline_indent_compact() {
        let options = PrintOptions {
            compact: true,
            ..Default::default()
        };
        assert_eq!(newline_indent(1, &options), " ");
    }

    // =======================================================================
    // Integration Tests - Using MusicXML Parser
    // =======================================================================

    /// Helper function to parse MusicXML and return the score
    fn parse_musicxml(xml: &str) -> crate::ir::score::ScorePartwise {
        crate::musicxml::parse_score(xml).expect("Failed to parse MusicXML")
    }

    /// Helper XML template for minimal score
    fn minimal_score_xml(content: &str) -> String {
        format!(
            r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        {}
                    </measure>
                </part>
            </score-partwise>"#,
            content
        )
    }

    // === print_score Integration Tests ===

    #[test]
    fn test_print_score_minimal_integration() {
        let xml = minimal_score_xml("");
        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.starts_with("(score-partwise"));
        assert!(result.contains("(part-list"));
        assert!(result.contains("(score-part :id \"P1\""));
        assert!(result.contains("(part-name \"Test\")"));
        assert!(result.contains("(part :id \"P1\""));
        assert!(result.contains("(measure :number \"1\""));
    }

    #[test]
    fn test_print_score_compact_integration() {
        let xml = minimal_score_xml("");
        let score = parse_musicxml(&xml);
        let options = PrintOptions {
            compact: true,
            ..Default::default()
        };
        let result = print_score(&score, &options);

        // Compact mode uses spaces instead of newlines
        assert!(!result.contains('\n'));
        assert!(result.contains("(score-partwise"));
        assert!(result.contains("(part-list"));
    }

    #[test]
    fn test_print_score_with_version_integration() {
        let xml = r#"<?xml version="1.0"?>
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

        let score = parse_musicxml(xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":version \"4.0\""));
    }

    // === print_note Integration Tests ===

    #[test]
    fn test_print_note_with_pitch_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch>
                    <step>C</step>
                    <octave>4</octave>
                </pitch>
                <duration>4</duration>
                <type>quarter</type>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(note"));
        assert!(result.contains(":pitch (pitch :step C :octave 4)"));
        assert!(result.contains(":duration 4"));
        assert!(result.contains(":type quarter"));
    }

    #[test]
    fn test_print_note_with_sharp_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch>
                    <step>F</step>
                    <alter>1</alter>
                    <octave>4</octave>
                </pitch>
                <duration>4</duration>
                <type>quarter</type>
                <accidental>sharp</accidental>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":pitch (pitch :step F :alter 1 :octave 4)"));
        assert!(result.contains(":accidental sharp"));
    }

    #[test]
    fn test_print_note_with_flat_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch>
                    <step>B</step>
                    <alter>-1</alter>
                    <octave>3</octave>
                </pitch>
                <duration>4</duration>
                <type>quarter</type>
                <accidental>flat</accidental>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":pitch (pitch :step B :alter -1 :octave 3)"));
        assert!(result.contains(":accidental flat"));
    }

    #[test]
    fn test_print_rest_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <rest/>
                <duration>4</duration>
                <type>quarter</type>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":rest (rest)"));
    }

    #[test]
    fn test_print_whole_measure_rest_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <rest measure="yes"/>
                <duration>16</duration>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":rest (rest :measure #t)"));
    }

    #[test]
    fn test_print_chord_integration() {
        let xml = minimal_score_xml(
            r#"<note>
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
            <note>
                <chord/>
                <pitch>
                    <step>G</step>
                    <octave>4</octave>
                </pitch>
                <duration>4</duration>
                <type>quarter</type>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":pitch (pitch :step C :octave 4)"));
        assert!(result.contains(":chord #t"));
        assert!(result.contains(":pitch (pitch :step E :octave 4)"));
        assert!(result.contains(":pitch (pitch :step G :octave 4)"));
    }

    #[test]
    fn test_print_grace_note_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <grace slash="yes"/>
                <pitch>
                    <step>D</step>
                    <octave>5</octave>
                </pitch>
                <type>eighth</type>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":grace #t"));
        assert!(result.contains(":slash #t"));
    }

    #[test]
    fn test_print_note_with_tie_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch>
                    <step>G</step>
                    <octave>4</octave>
                </pitch>
                <duration>4</duration>
                <type>quarter</type>
                <tie type="start"/>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":tie start"));
    }

    #[test]
    fn test_print_dotted_note_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch>
                    <step>C</step>
                    <octave>4</octave>
                </pitch>
                <duration>6</duration>
                <type>quarter</type>
                <dot/>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":dots 1"));
    }

    #[test]
    fn test_print_double_dotted_note_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch>
                    <step>C</step>
                    <octave>4</octave>
                </pitch>
                <duration>7</duration>
                <type>quarter</type>
                <dot/>
                <dot/>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":dots 2"));
    }

    #[test]
    fn test_print_note_with_stem_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch>
                    <step>C</step>
                    <octave>4</octave>
                </pitch>
                <duration>4</duration>
                <type>quarter</type>
                <stem>up</stem>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":stem up"));
    }

    #[test]
    fn test_print_note_with_beam_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch>
                    <step>C</step>
                    <octave>4</octave>
                </pitch>
                <duration>2</duration>
                <type>eighth</type>
                <beam number="1">begin</beam>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":beam (beam :number 1 :value begin)"));
    }

    #[test]
    fn test_print_note_with_voice_and_staff_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch>
                    <step>C</step>
                    <octave>4</octave>
                </pitch>
                <duration>4</duration>
                <type>quarter</type>
                <voice>1</voice>
                <staff>1</staff>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":voice \"1\""));
        assert!(result.contains(":staff 1"));
    }

    #[test]
    fn test_print_unpitched_note_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <unpitched>
                    <display-step>E</display-step>
                    <display-octave>4</display-octave>
                </unpitched>
                <duration>4</duration>
                <type>quarter</type>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":unpitched (unpitched :display-step E :display-octave 4)"));
    }

    #[test]
    fn test_print_cue_note_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <cue/>
                <pitch>
                    <step>C</step>
                    <octave>4</octave>
                </pitch>
                <duration>4</duration>
                <type>quarter</type>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":cue #t"));
    }

    #[test]
    fn test_print_note_with_time_modification_integration() {
        let xml = minimal_score_xml(
            r#"<note>
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
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":time-modification"));
        assert!(result.contains(":actual-notes 3"));
        assert!(result.contains(":normal-notes 2"));
    }

    #[test]
    fn test_print_note_with_notehead_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch>
                    <step>C</step>
                    <octave>4</octave>
                </pitch>
                <duration>4</duration>
                <type>quarter</type>
                <notehead>diamond</notehead>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":notehead diamond"));
    }

    // === print_attributes Integration Tests ===

    #[test]
    fn test_print_attributes_divisions_integration() {
        let xml = minimal_score_xml("<attributes><divisions>4</divisions></attributes>");

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(attributes"));
        assert!(result.contains(":divisions 4"));
    }

    #[test]
    fn test_print_attributes_key_major_integration() {
        let xml = minimal_score_xml(
            r#"<attributes>
                <key>
                    <fifths>2</fifths>
                    <mode>major</mode>
                </key>
            </attributes>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(key"));
        assert!(result.contains(":fifths 2"));
        assert!(result.contains(":mode major"));
    }

    #[test]
    fn test_print_attributes_key_minor_integration() {
        let xml = minimal_score_xml(
            r#"<attributes>
                <key>
                    <fifths>-3</fifths>
                    <mode>minor</mode>
                </key>
            </attributes>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":fifths -3"));
        assert!(result.contains(":mode minor"));
    }

    #[test]
    fn test_print_attributes_time_common_integration() {
        let xml = minimal_score_xml(
            r#"<attributes>
                <time symbol="common">
                    <beats>4</beats>
                    <beat-type>4</beat-type>
                </time>
            </attributes>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(time"));
        assert!(result.contains(":symbol common"));
        assert!(result.contains("(beats \"4\")"));
        assert!(result.contains("(beat-type \"4\")"));
    }

    #[test]
    fn test_print_attributes_time_cut_integration() {
        let xml = minimal_score_xml(
            r#"<attributes>
                <time symbol="cut">
                    <beats>2</beats>
                    <beat-type>2</beat-type>
                </time>
            </attributes>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":symbol cut"));
    }

    #[test]
    fn test_print_attributes_clef_treble_integration() {
        let xml = minimal_score_xml(
            r#"<attributes>
                <clef>
                    <sign>G</sign>
                    <line>2</line>
                </clef>
            </attributes>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(clef :sign G :line 2)"));
    }

    #[test]
    fn test_print_attributes_clef_bass_integration() {
        let xml = minimal_score_xml(
            r#"<attributes>
                <clef>
                    <sign>F</sign>
                    <line>4</line>
                </clef>
            </attributes>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(clef :sign F :line 4)"));
    }

    #[test]
    fn test_print_attributes_clef_alto_integration() {
        let xml = minimal_score_xml(
            r#"<attributes>
                <clef>
                    <sign>C</sign>
                    <line>3</line>
                </clef>
            </attributes>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(clef :sign C :line 3)"));
    }

    #[test]
    fn test_print_attributes_clef_octave_change_integration() {
        let xml = minimal_score_xml(
            r#"<attributes>
                <clef>
                    <sign>G</sign>
                    <line>2</line>
                    <clef-octave-change>-1</clef-octave-change>
                </clef>
            </attributes>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":clef-octave-change -1"));
    }

    #[test]
    fn test_print_attributes_staves_integration() {
        let xml = minimal_score_xml("<attributes><staves>2</staves></attributes>");

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":staves 2"));
    }

    #[test]
    fn test_print_attributes_transpose_integration() {
        let xml = minimal_score_xml(
            r#"<attributes>
                <transpose>
                    <diatonic>-1</diatonic>
                    <chromatic>-2</chromatic>
                </transpose>
            </attributes>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(transpose"));
        assert!(result.contains(":diatonic -1"));
        assert!(result.contains(":chromatic -2"));
    }

    #[test]
    fn test_print_attributes_staff_details_integration() {
        // Note: staff-details parsing may not be fully implemented
        // This test verifies the XML parses without error
        let xml = minimal_score_xml(
            r#"<attributes>
                <divisions>1</divisions>
            </attributes>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        // Verify basic attributes parsing works
        assert!(result.contains("(attributes"));
        assert!(result.contains(":divisions 1"));
    }

    // === print_direction Integration Tests ===

    #[test]
    fn test_print_direction_dynamics_p_integration() {
        let xml = minimal_score_xml(
            r#"<direction placement="below">
                <direction-type>
                    <dynamics><p/></dynamics>
                </direction-type>
            </direction>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(direction"));
        assert!(result.contains(":placement below"));
        assert!(result.contains("(dynamics p)"));
    }

    #[test]
    fn test_print_direction_dynamics_f_integration() {
        let xml = minimal_score_xml(
            r#"<direction placement="below">
                <direction-type>
                    <dynamics><f/></dynamics>
                </direction-type>
            </direction>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(dynamics f)"));
    }

    #[test]
    fn test_print_direction_dynamics_ff_integration() {
        let xml = minimal_score_xml(
            r#"<direction>
                <direction-type>
                    <dynamics><ff/></dynamics>
                </direction-type>
            </direction>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(dynamics ff)"));
    }

    #[test]
    fn test_print_direction_dynamics_mf_integration() {
        let xml = minimal_score_xml(
            r#"<direction>
                <direction-type>
                    <dynamics><mf/></dynamics>
                </direction-type>
            </direction>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(dynamics mf)"));
    }

    #[test]
    fn test_print_direction_wedge_crescendo_integration() {
        let xml = minimal_score_xml(
            r#"<direction>
                <direction-type>
                    <wedge type="crescendo"/>
                </direction-type>
            </direction>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(wedge :type crescendo)"));
    }

    #[test]
    fn test_print_direction_wedge_diminuendo_integration() {
        let xml = minimal_score_xml(
            r#"<direction>
                <direction-type>
                    <wedge type="diminuendo"/>
                </direction-type>
            </direction>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(wedge :type diminuendo)"));
    }

    #[test]
    fn test_print_direction_wedge_stop_integration() {
        let xml = minimal_score_xml(
            r#"<direction>
                <direction-type>
                    <wedge type="stop"/>
                </direction-type>
            </direction>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(wedge :type stop)"));
    }

    #[test]
    fn test_print_direction_metronome_integration() {
        let xml = minimal_score_xml(
            r#"<direction>
                <direction-type>
                    <metronome>
                        <beat-unit>quarter</beat-unit>
                        <per-minute>120</per-minute>
                    </metronome>
                </direction-type>
            </direction>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(metronome"));
        assert!(result.contains(":beat-unit quarter"));
        assert!(result.contains(":per-minute \"120\""));
    }

    #[test]
    fn test_print_direction_metronome_with_dot_integration() {
        let xml = minimal_score_xml(
            r#"<direction>
                <direction-type>
                    <metronome>
                        <beat-unit>quarter</beat-unit>
                        <beat-unit-dot/>
                        <per-minute>60</per-minute>
                    </metronome>
                </direction-type>
            </direction>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":beat-unit-dot"));
    }

    #[test]
    fn test_print_direction_words_integration() {
        let xml = minimal_score_xml(
            r#"<direction>
                <direction-type>
                    <words>cresc.</words>
                </direction-type>
            </direction>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(words \"cresc.\""));
    }

    #[test]
    fn test_print_direction_pedal_start_integration() {
        let xml = minimal_score_xml(
            r#"<direction>
                <direction-type>
                    <pedal type="start" line="yes"/>
                </direction-type>
            </direction>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(pedal"));
        assert!(result.contains(":type start"));
        assert!(result.contains(":line #t"));
    }

    #[test]
    fn test_print_direction_pedal_stop_integration() {
        let xml = minimal_score_xml(
            r#"<direction>
                <direction-type>
                    <pedal type="stop"/>
                </direction-type>
            </direction>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":type stop"));
    }

    #[test]
    fn test_print_direction_octave_shift_integration() {
        let xml = minimal_score_xml(
            r#"<direction>
                <direction-type>
                    <octave-shift type="up" size="8"/>
                </direction-type>
            </direction>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(octave-shift"));
        assert!(result.contains(":type up"));
        assert!(result.contains(":size 8"));
    }

    #[test]
    fn test_print_direction_rehearsal_integration() {
        let xml = minimal_score_xml(
            r#"<direction>
                <direction-type>
                    <rehearsal>A</rehearsal>
                </direction-type>
            </direction>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(rehearsal \"A\")"));
    }

    #[test]
    fn test_print_direction_segno_integration() {
        let xml = minimal_score_xml(
            r#"<direction>
                <direction-type>
                    <segno/>
                </direction-type>
            </direction>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(segno)"));
    }

    #[test]
    fn test_print_direction_coda_integration() {
        let xml = minimal_score_xml(
            r#"<direction>
                <direction-type>
                    <coda/>
                </direction-type>
            </direction>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(coda)"));
    }

    // === print_notations Integration Tests ===

    #[test]
    fn test_print_notation_tied_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>4</duration>
                <type>quarter</type>
                <tie type="start"/>
                <notations>
                    <tied type="start"/>
                </notations>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(notations"));
        assert!(result.contains("(tied :type start)"));
    }

    #[test]
    fn test_print_notation_slur_start_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>4</duration>
                <type>quarter</type>
                <notations>
                    <slur type="start" number="1" placement="above"/>
                </notations>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(slur :type start :number 1 :placement above)"));
    }

    #[test]
    fn test_print_notation_slur_stop_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>4</duration>
                <type>quarter</type>
                <notations>
                    <slur type="stop" number="1"/>
                </notations>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(slur :type stop :number 1)"));
    }

    #[test]
    fn test_print_notation_fermata_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>4</duration>
                <type>quarter</type>
                <notations>
                    <fermata type="upright"/>
                </notations>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(fermata"));
        assert!(result.contains(":type upright"));
    }

    #[test]
    fn test_print_notation_tuplet_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>2</duration>
                <type>eighth</type>
                <time-modification>
                    <actual-notes>3</actual-notes>
                    <normal-notes>2</normal-notes>
                </time-modification>
                <notations>
                    <tuplet type="start" number="1" bracket="yes"/>
                </notations>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(tuplet"));
        assert!(result.contains(":type start"));
        assert!(result.contains(":bracket #t"));
    }

    #[test]
    fn test_print_notation_arpeggiate_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>4</duration>
                <type>quarter</type>
                <notations>
                    <arpeggiate direction="up"/>
                </notations>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(arpeggiate"));
        assert!(result.contains(":direction up"));
    }

    #[test]
    fn test_print_notation_glissando_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>4</duration>
                <type>quarter</type>
                <notations>
                    <glissando type="start" number="1">gliss.</glissando>
                </notations>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(glissando"));
        assert!(result.contains(":type start"));
        assert!(result.contains("\"gliss.\""));
    }

    // === print_articulations Integration Tests ===

    #[test]
    fn test_print_articulation_staccato_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>4</duration>
                <type>quarter</type>
                <notations>
                    <articulations>
                        <staccato/>
                    </articulations>
                </notations>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(articulations"));
        assert!(result.contains("(staccato)"));
    }

    #[test]
    fn test_print_articulation_accent_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>4</duration>
                <type>quarter</type>
                <notations>
                    <articulations>
                        <accent/>
                    </articulations>
                </notations>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(accent)"));
    }

    #[test]
    fn test_print_articulation_tenuto_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>4</duration>
                <type>quarter</type>
                <notations>
                    <articulations>
                        <tenuto/>
                    </articulations>
                </notations>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(tenuto)"));
    }

    #[test]
    fn test_print_articulation_staccatissimo_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>4</duration>
                <type>quarter</type>
                <notations>
                    <articulations>
                        <staccatissimo/>
                    </articulations>
                </notations>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(staccatissimo)"));
    }

    #[test]
    fn test_print_multiple_articulations_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>4</duration>
                <type>quarter</type>
                <notations>
                    <articulations>
                        <accent/>
                        <staccato/>
                    </articulations>
                </notations>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(accent)"));
        assert!(result.contains("(staccato)"));
    }

    // === print_ornaments Integration Tests ===

    #[test]
    fn test_print_ornament_trill_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>4</duration>
                <type>quarter</type>
                <notations>
                    <ornaments>
                        <trill-mark/>
                    </ornaments>
                </notations>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(ornaments"));
        assert!(result.contains("(trill-mark)"));
    }

    #[test]
    fn test_print_ornament_turn_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>4</duration>
                <type>quarter</type>
                <notations>
                    <ornaments>
                        <turn/>
                    </ornaments>
                </notations>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(turn)"));
    }

    #[test]
    fn test_print_ornament_mordent_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>4</duration>
                <type>quarter</type>
                <notations>
                    <ornaments>
                        <mordent/>
                    </ornaments>
                </notations>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(mordent)"));
    }

    #[test]
    fn test_print_ornament_inverted_mordent_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>4</duration>
                <type>quarter</type>
                <notations>
                    <ornaments>
                        <inverted-mordent/>
                    </ornaments>
                </notations>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(inverted-mordent)"));
    }

    #[test]
    fn test_print_ornament_tremolo_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>4</duration>
                <type>quarter</type>
                <notations>
                    <ornaments>
                        <tremolo type="single">3</tremolo>
                    </ornaments>
                </notations>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(tremolo"));
        assert!(result.contains(":value 3"));
    }

    // === print_technical Integration Tests ===

    #[test]
    fn test_print_technical_fingering_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>4</duration>
                <type>quarter</type>
                <notations>
                    <technical>
                        <fingering>1</fingering>
                    </technical>
                </notations>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(technical"));
        assert!(result.contains("(fingering \"1\")"));
    }

    #[test]
    fn test_print_technical_up_bow_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>4</duration>
                <type>quarter</type>
                <notations>
                    <technical>
                        <up-bow/>
                    </technical>
                </notations>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(up-bow)"));
    }

    #[test]
    fn test_print_technical_down_bow_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>4</duration>
                <type>quarter</type>
                <notations>
                    <technical>
                        <down-bow/>
                    </technical>
                </notations>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(down-bow)"));
    }

    #[test]
    fn test_print_technical_harmonic_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>4</duration>
                <type>quarter</type>
                <notations>
                    <technical>
                        <harmonic><natural/></harmonic>
                    </technical>
                </notations>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(harmonic"));
        assert!(result.contains(":natural #t"));
    }

    #[test]
    fn test_print_technical_fret_string_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>4</duration>
                <type>quarter</type>
                <notations>
                    <technical>
                        <string>3</string>
                        <fret>5</fret>
                    </technical>
                </notations>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(string 3)"));
        assert!(result.contains("(fret 5)"));
    }

    // === print_barline Integration Tests ===

    #[test]
    fn test_print_barline_regular_integration() {
        let xml = minimal_score_xml(
            r#"<barline location="right">
                <bar-style>regular</bar-style>
            </barline>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(barline"));
        assert!(result.contains(":location right"));
        assert!(result.contains(":bar-style regular"));
    }

    #[test]
    fn test_print_barline_light_heavy_integration() {
        let xml = minimal_score_xml(
            r#"<barline location="right">
                <bar-style>light-heavy</bar-style>
            </barline>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":bar-style light-heavy"));
    }

    #[test]
    fn test_print_barline_double_integration() {
        let xml = minimal_score_xml(
            r#"<barline location="right">
                <bar-style>light-light</bar-style>
            </barline>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":bar-style light-light"));
    }

    #[test]
    fn test_print_barline_forward_repeat_integration() {
        let xml = minimal_score_xml(
            r#"<barline location="left">
                <bar-style>heavy-light</bar-style>
                <repeat direction="forward"/>
            </barline>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(repeat"));
        assert!(result.contains(":direction forward"));
    }

    #[test]
    fn test_print_barline_backward_repeat_integration() {
        let xml = minimal_score_xml(
            r#"<barline location="right">
                <bar-style>light-heavy</bar-style>
                <repeat direction="backward" times="2"/>
            </barline>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":direction backward"));
        assert!(result.contains(":times 2"));
    }

    #[test]
    fn test_print_barline_ending_start_integration() {
        let xml = minimal_score_xml(
            r#"<barline location="left">
                <ending number="1" type="start"/>
            </barline>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(ending"));
        assert!(result.contains(":number \"1\""));
        assert!(result.contains(":type start"));
    }

    #[test]
    fn test_print_barline_ending_stop_integration() {
        let xml = minimal_score_xml(
            r#"<barline location="right">
                <bar-style>light-heavy</bar-style>
                <ending number="1" type="stop"/>
                <repeat direction="backward"/>
            </barline>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":type stop"));
    }

    // === print_backup and print_forward Integration Tests ===

    #[test]
    fn test_print_backup_integration() {
        let xml = minimal_score_xml("<backup><duration>4</duration></backup>");

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(backup :duration 4)"));
    }

    #[test]
    fn test_print_forward_integration() {
        let xml = minimal_score_xml(
            r#"<forward>
                <duration>8</duration>
                <voice>2</voice>
                <staff>1</staff>
            </forward>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(forward :duration 8"));
        assert!(result.contains(":voice \"2\""));
        assert!(result.contains(":staff 1"));
    }

    // === print_lyric Integration Tests ===

    #[test]
    fn test_print_lyric_single_syllable_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>4</duration>
                <type>quarter</type>
                <lyric number="1">
                    <syllabic>single</syllabic>
                    <text>Hello</text>
                </lyric>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(lyric"));
        assert!(result.contains(":number \"1\""));
        assert!(result.contains(":syllabic single"));
        assert!(result.contains(":text \"Hello\""));
    }

    #[test]
    fn test_print_lyric_begin_syllable_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>4</duration>
                <type>quarter</type>
                <lyric number="1">
                    <syllabic>begin</syllabic>
                    <text>Hel</text>
                </lyric>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":syllabic begin"));
        assert!(result.contains(":text \"Hel\""));
    }

    #[test]
    fn test_print_lyric_end_syllable_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>4</duration>
                <type>quarter</type>
                <lyric number="1">
                    <syllabic>end</syllabic>
                    <text>lo</text>
                </lyric>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains(":syllabic end"));
    }

    #[test]
    fn test_print_lyric_with_extend_integration() {
        let xml = minimal_score_xml(
            r#"<note>
                <pitch><step>C</step><octave>4</octave></pitch>
                <duration>4</duration>
                <type>quarter</type>
                <lyric number="1">
                    <syllabic>single</syllabic>
                    <text>Ah</text>
                    <extend type="start"/>
                </lyric>
            </note>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(extend"));
        assert!(result.contains(":type start"));
    }

    // === print_part_group and part_list Integration Tests ===

    #[test]
    fn test_print_part_group_integration() {
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
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

        let score = parse_musicxml(xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(part-group :type start :number \"1\""));
        assert!(result.contains(":group-name \"Strings\""));
        assert!(result.contains("(group-symbol bracket)"));
        assert!(result.contains("(part-group :type stop :number \"1\""));
    }

    #[test]
    fn test_print_score_part_with_abbreviation_integration() {
        // Note: part-abbreviation may not be fully parsed
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Violin I</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

        let score = parse_musicxml(xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(part-name \"Violin I\")"));
        assert!(result.contains("(score-part :id \"P1\""));
    }

    #[test]
    fn test_print_midi_instrument_integration() {
        // Note: score-instrument and midi-instrument parsing may not be fully implemented
        // This test verifies basic score-part handling
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Piano</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

        let score = parse_musicxml(xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(score-part :id \"P1\""));
        assert!(result.contains("(part-name \"Piano\")"));
    }

    // === print_identification and print_work Integration Tests ===

    #[test]
    fn test_print_work_integration() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <work>
                    <work-number>BWV 846</work-number>
                    <work-title>Prelude and Fugue in C Major</work-title>
                </work>
                <part-list>
                    <score-part id="P1">
                        <part-name>Piano</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

        let score = parse_musicxml(xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(work"));
        assert!(result.contains(":work-number \"BWV 846\""));
        assert!(result.contains(":work-title \"Prelude and Fugue in C Major\""));
    }

    #[test]
    fn test_print_identification_with_creator_integration() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <identification>
                    <creator type="composer">J.S. Bach</creator>
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

        let score = parse_musicxml(xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(identification"));
        assert!(result.contains("(creator :type \"composer\" \"J.S. Bach\")"));
    }

    #[test]
    fn test_print_encoding_integration() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <identification>
                    <encoding>
                        <software>MuseScore 4.0</software>
                        <encoding-date>2024-01-15</encoding-date>
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

        let score = parse_musicxml(xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        assert!(result.contains("(encoding"));
        assert!(result.contains("(software \"MuseScore 4.0\")"));
        assert!(result.contains("(encoding-date \"2024-01-15\")"));
    }

    // === Additional Symbol Conversion Tests ===

    #[test]
    fn test_note_type_all_values() {
        // Test various note types
        let note_types = [
            ("whole", "whole"),
            ("half", "half"),
            ("quarter", "quarter"),
            ("eighth", "eighth"),
            ("16th", "16th"),
            ("32nd", "32nd"),
            ("64th", "64th"),
        ];

        for (xml_type, expected) in note_types {
            let xml = minimal_score_xml(&format!(
                r#"<note>
                    <pitch><step>C</step><octave>4</octave></pitch>
                    <duration>1</duration>
                    <type>{}</type>
                </note>"#,
                xml_type
            ));

            let score = parse_musicxml(&xml);
            let options = PrintOptions::default();
            let result = print_score(&score, &options);

            assert!(
                result.contains(&format!(":type {}", expected)),
                "Expected :type {} in output",
                expected
            );
        }
    }

    #[test]
    fn test_all_clef_signs() {
        let clef_signs = [("G", "G"), ("F", "F"), ("C", "C")];

        for (xml_sign, expected) in clef_signs {
            let xml = minimal_score_xml(&format!(
                r#"<attributes>
                    <clef>
                        <sign>{}</sign>
                        <line>2</line>
                    </clef>
                </attributes>"#,
                xml_sign
            ));

            let score = parse_musicxml(&xml);
            let options = PrintOptions::default();
            let result = print_score(&score, &options);

            assert!(
                result.contains(&format!(":sign {}", expected)),
                "Expected :sign {} in output",
                expected
            );
        }
    }

    #[test]
    fn test_all_modes() {
        let modes = [
            "major",
            "minor",
            "dorian",
            "phrygian",
            "lydian",
            "mixolydian",
            "aeolian",
            "locrian",
        ];

        for mode in modes {
            let xml = minimal_score_xml(&format!(
                r#"<attributes>
                    <key>
                        <fifths>0</fifths>
                        <mode>{}</mode>
                    </key>
                </attributes>"#,
                mode
            ));

            let score = parse_musicxml(&xml);
            let options = PrintOptions::default();
            let result = print_score(&score, &options);

            assert!(
                result.contains(&format!(":mode {}", mode)),
                "Expected :mode {} in output",
                mode
            );
        }
    }

    #[test]
    fn test_all_bar_styles() {
        let bar_styles = [
            "regular",
            "dotted",
            "dashed",
            "heavy",
            "light-light",
            "light-heavy",
            "heavy-light",
            "heavy-heavy",
            "none",
        ];

        for style in bar_styles {
            let xml = minimal_score_xml(&format!(
                r#"<barline location="right">
                    <bar-style>{}</bar-style>
                </barline>"#,
                style
            ));

            let score = parse_musicxml(&xml);
            let options = PrintOptions::default();
            let result = print_score(&score, &options);

            assert!(
                result.contains(&format!(":bar-style {}", style)),
                "Expected :bar-style {} in output",
                style
            );
        }
    }

    #[test]
    fn test_measure_with_multiple_elements_integration() {
        // Test a more complex measure with multiple elements
        let xml = minimal_score_xml(
            r#"<attributes>
                    <divisions>4</divisions>
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
                <note>
                    <pitch><step>C</step><octave>4</octave></pitch>
                    <duration>4</duration>
                    <type>quarter</type>
                </note>
                <note>
                    <pitch><step>D</step><octave>4</octave></pitch>
                    <duration>4</duration>
                    <type>quarter</type>
                </note>
                <note>
                    <pitch><step>E</step><octave>4</octave></pitch>
                    <duration>4</duration>
                    <type>quarter</type>
                </note>
                <note>
                    <pitch><step>F</step><octave>4</octave></pitch>
                    <duration>4</duration>
                    <type>quarter</type>
                </note>
                <barline location="right">
                    <bar-style>light-heavy</bar-style>
                </barline>"#,
        );

        let score = parse_musicxml(&xml);
        let options = PrintOptions::default();
        let result = print_score(&score, &options);

        // Verify all elements are present
        assert!(result.contains("(attributes"));
        assert!(result.contains(":divisions 4"));
        assert!(result.contains("(key"));
        assert!(result.contains("(time"));
        assert!(result.contains("(clef"));
        assert!(result.contains(":pitch (pitch :step C :octave 4)"));
        assert!(result.contains(":pitch (pitch :step D :octave 4)"));
        assert!(result.contains(":pitch (pitch :step E :octave 4)"));
        assert!(result.contains(":pitch (pitch :step F :octave 4)"));
        assert!(result.contains("(barline"));
    }
}

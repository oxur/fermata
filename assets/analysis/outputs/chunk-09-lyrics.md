# Chunk 9: Lyrics

## Overview

Lyrics in MusicXML represent text underlays for vocal music. The lyric system handles multiple verses, syllable hyphenation, melismas (extended notes), elisions (multiple syllables on one note), and specialized vocal effects like laughing and humming.

**Key Concept: Syllabic handling.** A word like "mu-sic" is split into syllables. Each syllable has a `<syllabic>` type:
- `single` — complete word ("the", "and")
- `begin` — first syllable ("mu-")
- `middle` — middle syllable (in "beau-ti-ful", the "ti-")
- `end` — last syllable ("-sic", "-ful")

**Key Concept: Multiple verses.** Lyrics can have multiple numbered lines (verse 1, verse 2, chorus) identified by the `number` or `name` attribute.

**Key Concept: Melisma.** When a syllable extends across multiple notes, an `<extend>` element indicates the continuation line.

## MusicXML Elements Covered

| Element | Type | Required Children | Optional Children | Key Attributes |
|---------|------|-------------------|-------------------|----------------|
| `<lyric>` | complex | See content model | `<end-line>`, `<end-paragraph>` | `number`, `name`, `justify`, `placement`, print-style |
| `<syllabic>` | simple (enum) | — | — | — |
| `<text>` | complex | text content | — | font, color, text-decoration, text-rotation, letter-spacing, lang, text-direction |
| `<elision>` | complex | text content | — | font, color, smufl |
| `<extend>` | complex | — | — | `type`, position, color |
| `<laughing>` | empty | — | — | — |
| `<humming>` | empty | — | — | — |
| `<end-line>` | empty | — | — | — |
| `<end-paragraph>` | empty | — | — | — |

## S-Expression Mappings

### lyric

The lyric element contains the text and syllabic information for a note's lyric underlay.

**MusicXML:**
```xml
<note>
  <pitch><step>C</step><octave>4</octave></pitch>
  <duration>1</duration>
  <type>quarter</type>
  <lyric number="1">
    <syllabic>single</syllabic>
    <text>love</text>
  </lyric>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :C :octave 4)
  :duration 1
  :type :quarter
  :lyrics ((lyric :number "1"
             :syllabic :single
             :text "love")))
```

**Multiple verses:**
```xml
<lyric number="1">
  <syllabic>single</syllabic>
  <text>love</text>
</lyric>
<lyric number="2">
  <syllabic>single</syllabic>
  <text>peace</text>
</lyric>
```

```lisp
:lyrics ((lyric :number "1" :syllabic :single :text "love")
         (lyric :number "2" :syllabic :single :text "peace"))
```

**Mapping Rules:**
- Multiple `<lyric>` elements → list of `(lyric ...)` forms
- `number` attribute → `:number` (string)
- `name` attribute → `:name` (string, e.g., "verse", "chorus")
- `justify` attribute → `:justify` (`:left`, `:center`, `:right`)
- `placement` attribute → `:placement` (`:above`, `:below`)

**Type Definition:**
```rust
pub struct Lyric {
    pub number: Option<String>,
    pub name: Option<String>,
    pub justify: Option<LeftCenterRight>,
    pub placement: Option<AboveBelow>,
    // print-style attributes
    pub default_x: Option<Tenths>,
    pub default_y: Option<Tenths>,
    pub relative_x: Option<Tenths>,
    pub relative_y: Option<Tenths>,
    pub color: Option<Color>,

    pub content: LyricContent,
    pub end_line: bool,
    pub end_paragraph: bool,
}

pub enum LyricContent {
    /// Normal syllable with optional extensions
    Syllable {
        syllabic: Option<Syllabic>,
        text: TextElementData,
        /// Additional syllables connected by elisions
        extensions: Vec<LyricExtension>,
        extend: Option<Extend>,
    },
    /// Extend-only (melisma continuation)
    ExtendOnly(Extend),
    /// Laughing voice indicator
    Laughing,
    /// Humming voice indicator
    Humming,
}

pub struct LyricExtension {
    pub elision: Option<Elision>,
    pub syllabic: Option<Syllabic>,
    pub text: TextElementData,
}
```

---

### syllabic

Indicates the position of a syllable within a word.

**MusicXML:**
```xml
<syllabic>begin</syllabic>   <!-- First syllable: "mu-" -->
<syllabic>middle</syllabic>  <!-- Middle syllable: "-si-" -->
<syllabic>end</syllabic>     <!-- Last syllable: "-cal" -->
<syllabic>single</syllabic>  <!-- Complete word: "song" -->
```

**S-expr:**
```lisp
:syllabic :begin
:syllabic :middle
:syllabic :end
:syllabic :single
```

**Values:**
- `:single` — Complete word, no hyphen
- `:begin` — First syllable, followed by hyphen
- `:middle` — Middle syllable, surrounded by hyphens
- `:end` — Final syllable, preceded by hyphen

**Type Definition:**
```rust
pub enum Syllabic {
    Single,
    Begin,
    Middle,
    End,
}
```

---

### text

The text element contains the actual syllable or word content with optional formatting.

**MusicXML:**
```xml
<text>love</text>
<text font-weight="bold" color="#FF0000">SHOUT</text>
<text xml:lang="it">a</text>
```

**S-expr:**
```lisp
:text "love"
:text (text :value "SHOUT" :font-weight :bold :color "#FF0000")
:text (text :value "a" :lang "it")
```

**Simple form (no attributes):**
```lisp
:text "love"
```

**With attributes:**
```lisp
:text (text :value "SHOUT"
            :font-weight :bold
            :color "#FF0000")
```

**Mapping Rules:**
- Simple text → string value
- Text with formatting → `(text :value "..." :attr val)`
- `xml:lang` → `:lang`
- Font attributes → `:font-family`, `:font-style`, `:font-size`, `:font-weight`
- Text decoration → `:underline`, `:overline`, `:line-through`

**Type Definition:**
```rust
pub struct TextElementData {
    pub value: String,
    // Font attributes
    pub font_family: Option<String>,
    pub font_style: Option<FontStyle>,
    pub font_size: Option<FontSize>,
    pub font_weight: Option<FontWeight>,
    // Color
    pub color: Option<Color>,
    // Text decoration
    pub underline: Option<u8>,
    pub overline: Option<u8>,
    pub line_through: Option<u8>,
    // Other
    pub rotation: Option<RotationDegrees>,
    pub letter_spacing: Option<LetterSpacing>,
    pub lang: Option<String>,
    pub direction: Option<TextDirection>,
}
```

---

### elision

Connects multiple syllables sung on a single note (e.g., "heav'n" sung as one syllable).

**MusicXML:**
```xml
<lyric number="1">
  <syllabic>single</syllabic>
  <text>heav</text>
  <elision>'</elision>
  <text>n</text>
</lyric>
```

**S-expr:**
```lisp
(lyric :number "1"
  :syllabic :single
  :text "heav"
  :extensions ((elision :value "'") (text "n")))
```

**Alternative representation:**
```lisp
(lyric :number "1"
  :syllabic :single
  :text "heav"
  :elision "'"
  :text-2 "n")
```

**Empty elision (common for connecting syllables):**
```xml
<elision/>
```

```lisp
:elision ""
```

**Mapping Rules:**
- Elision text → `:value` (often an apostrophe or underscore)
- Empty elision → empty string
- Multiple elision-text pairs → `:extensions` list
- SMuFL glyph → `:smufl` attribute

**Type Definition:**
```rust
pub struct Elision {
    pub value: String,  // Often empty, apostrophe, or underscore
    pub font_family: Option<String>,
    pub font_style: Option<FontStyle>,
    pub font_size: Option<FontSize>,
    pub font_weight: Option<FontWeight>,
    pub color: Option<Color>,
    pub smufl: Option<SmuflLyricsGlyphName>,
}
```

---

### extend

Indicates a melisma — a syllable that extends across multiple notes. The line is typically drawn under the notes.

**MusicXML:**
```xml
<!-- First note of melisma -->
<note>
  <pitch><step>C</step><octave>4</octave></pitch>
  <duration>1</duration>
  <lyric number="1">
    <syllabic>single</syllabic>
    <text>love</text>
    <extend type="start"/>
  </lyric>
</note>

<!-- Middle note(s) of melisma -->
<note>
  <pitch><step>D</step><octave>4</octave></pitch>
  <duration>1</duration>
  <lyric number="1">
    <extend type="continue"/>
  </lyric>
</note>

<!-- Last note of melisma -->
<note>
  <pitch><step>E</step><octave>4</octave></pitch>
  <duration>1</duration>
  <lyric number="1">
    <extend type="stop"/>
  </lyric>
</note>
```

**S-expr:**
```lisp
;; First note
(note
  :pitch (pitch :step :C :octave 4)
  :duration 1
  :lyrics ((lyric :number "1"
             :syllabic :single
             :text "love"
             :extend (extend :type :start))))

;; Middle note
(note
  :pitch (pitch :step :D :octave 4)
  :duration 1
  :lyrics ((lyric :number "1"
             :extend (extend :type :continue))))

;; Last note
(note
  :pitch (pitch :step :E :octave 4)
  :duration 1
  :lyrics ((lyric :number "1"
             :extend (extend :type :stop))))
```

**Mapping Rules:**
- `type` attribute → `:type` (`:start`, `:stop`, `:continue`)
- Position attributes → `:default-x`, `:default-y`, etc.
- Old-style extend without type → omit `:type` (implies one-note extend)

**Type Definition:**
```rust
pub struct Extend {
    pub r#type: Option<StartStopContinue>,
    pub default_x: Option<Tenths>,
    pub default_y: Option<Tenths>,
    pub relative_x: Option<Tenths>,
    pub relative_y: Option<Tenths>,
    pub color: Option<Color>,
}
```

---

### laughing and humming

Special vocal effects that replace normal syllables.

**MusicXML:**
```xml
<lyric number="1">
  <laughing/>
</lyric>

<lyric number="1">
  <humming/>
</lyric>
```

**S-expr:**
```lisp
(lyric :number "1" :laughing t)

(lyric :number "1" :humming t)
```

**Type Definition:**
```rust
// Part of LyricContent enum:
pub enum LyricContent {
    // ...
    Laughing,
    Humming,
}
```

---

### end-line and end-paragraph

From MIDI lyric meta-events (RP-017), used for karaoke and lyric display applications.

**MusicXML:**
```xml
<lyric number="1">
  <syllabic>end</syllabic>
  <text>song</text>
  <end-line/>
</lyric>

<lyric number="1">
  <syllabic>end</syllabic>
  <text>verse</text>
  <end-paragraph/>
</lyric>
```

**S-expr:**
```lisp
(lyric :number "1"
  :syllabic :end
  :text "song"
  :end-line t)

(lyric :number "1"
  :syllabic :end
  :text "verse"
  :end-paragraph t)
```

---

## Interrelationships

### lyric ↔ note

Lyrics are children of notes. Each note can have multiple lyrics (multiple verses):

```lisp
(note
  :pitch (pitch :step :C :octave 4)
  :duration 1
  :type :quarter
  :lyrics ((lyric :number "1" :text "first")
           (lyric :number "2" :text "second")
           (lyric :name "chorus" :text "refrain")))
```

### syllabic ↔ hyphenation

The `syllabic` value determines whether a hyphen is rendered:
- `begin` and `middle` → hyphen after
- `middle` and `end` → hyphen before (implied by previous syllable)
- `single` → no hyphens

### extend ↔ melisma

An extend line is drawn when a syllable spans multiple notes:
1. First note: syllable + `extend type="start"`
2. Middle notes: `extend type="continue"` only (no text)
3. Last note: `extend type="stop"`

### elision ↔ multiple syllables

Elision connects syllables sung on one note without hyphenation. Common for:
- Contractions: "heav'n" (heaven)
- Multi-syllable words on one note
- Foreign language elisions

---

## Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Multiple lyrics | List of `(lyric ...)` forms | Matches MusicXML; allows multiple verses |
| Lyric identification | Both `:number` and `:name` | Some scores use numbers, others use names |
| Simple text shorthand | `:text "word"` | Most lyrics have no formatting |
| Elision handling | `:extensions` list | Complex elisions have multiple text elements |
| Extend types as keywords | `:start`, `:stop`, `:continue` | Clear, matches MusicXML |
| Laughing/humming as flags | `:laughing t` / `:humming t` | Simple boolean; replaces syllable content |

---

## Open Questions

- [ ] **Elision representation** — The current `:extensions` approach is accurate but verbose. Should there be a simpler syntax for common cases like `"heav'n"`?

- [ ] **Print-lyric attribute** — MusicXML has `print-lyric` on notes to suppress lyric printing. Should this be on the note or lyric level in IR?

- [ ] **Lyric language** — `<lyric>` can have `xml:lang` for the entire lyric, but `<text>` can override. How to handle mixed-language lyrics?

---

## Examples

### Example 1: Simple Single-Syllable Words

**Musical meaning:** "I love you" on three quarter notes

**MusicXML:**
```xml
<note>
  <pitch><step>C</step><octave>4</octave></pitch>
  <duration>1</duration>
  <type>quarter</type>
  <lyric number="1">
    <syllabic>single</syllabic>
    <text>I</text>
  </lyric>
</note>
<note>
  <pitch><step>D</step><octave>4</octave></pitch>
  <duration>1</duration>
  <type>quarter</type>
  <lyric number="1">
    <syllabic>single</syllabic>
    <text>love</text>
  </lyric>
</note>
<note>
  <pitch><step>E</step><octave>4</octave></pitch>
  <duration>1</duration>
  <type>quarter</type>
  <lyric number="1">
    <syllabic>single</syllabic>
    <text>you</text>
  </lyric>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :C :octave 4)
  :duration 1
  :type :quarter
  :lyrics ((lyric :number "1" :syllabic :single :text "I")))
(note
  :pitch (pitch :step :D :octave 4)
  :duration 1
  :type :quarter
  :lyrics ((lyric :number "1" :syllabic :single :text "love")))
(note
  :pitch (pitch :step :E :octave 4)
  :duration 1
  :type :quarter
  :lyrics ((lyric :number "1" :syllabic :single :text "you")))
```

---

### Example 2: Multi-Syllable Word with Hyphenation

**Musical meaning:** "mu-sic" spanning two notes

**MusicXML:**
```xml
<note>
  <pitch><step>G</step><octave>4</octave></pitch>
  <duration>1</duration>
  <type>quarter</type>
  <lyric number="1">
    <syllabic>begin</syllabic>
    <text>mu</text>
  </lyric>
</note>
<note>
  <pitch><step>A</step><octave>4</octave></pitch>
  <duration>1</duration>
  <type>quarter</type>
  <lyric number="1">
    <syllabic>end</syllabic>
    <text>sic</text>
  </lyric>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :G :octave 4)
  :duration 1
  :type :quarter
  :lyrics ((lyric :number "1" :syllabic :begin :text "mu")))
(note
  :pitch (pitch :step :A :octave 4)
  :duration 1
  :type :quarter
  :lyrics ((lyric :number "1" :syllabic :end :text "sic")))
```

---

### Example 3: Melisma with Extend Line

**Musical meaning:** "love" sung across three notes with melisma line

**MusicXML:**
```xml
<note>
  <pitch><step>C</step><octave>5</octave></pitch>
  <duration>1</duration>
  <type>quarter</type>
  <lyric number="1">
    <syllabic>single</syllabic>
    <text>love</text>
    <extend type="start"/>
  </lyric>
</note>
<note>
  <pitch><step>D</step><octave>5</octave></pitch>
  <duration>1</duration>
  <type>quarter</type>
  <lyric number="1">
    <extend type="continue"/>
  </lyric>
</note>
<note>
  <pitch><step>E</step><octave>5</octave></pitch>
  <duration>1</duration>
  <type>quarter</type>
  <lyric number="1">
    <extend type="stop"/>
  </lyric>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :C :octave 5)
  :duration 1
  :type :quarter
  :lyrics ((lyric :number "1"
             :syllabic :single
             :text "love"
             :extend (extend :type :start))))
(note
  :pitch (pitch :step :D :octave 5)
  :duration 1
  :type :quarter
  :lyrics ((lyric :number "1"
             :extend (extend :type :continue))))
(note
  :pitch (pitch :step :E :octave 5)
  :duration 1
  :type :quarter
  :lyrics ((lyric :number "1"
             :extend (extend :type :stop))))
```

---

### Example 4: Multiple Verses

**Musical meaning:** Two verses with different lyrics on the same melody

**MusicXML:**
```xml
<note>
  <pitch><step>C</step><octave>4</octave></pitch>
  <duration>1</duration>
  <type>quarter</type>
  <lyric number="1">
    <syllabic>single</syllabic>
    <text>Day</text>
  </lyric>
  <lyric number="2">
    <syllabic>single</syllabic>
    <text>Night</text>
  </lyric>
</note>
<note>
  <pitch><step>E</step><octave>4</octave></pitch>
  <duration>1</duration>
  <type>quarter</type>
  <lyric number="1">
    <syllabic>single</syllabic>
    <text>light</text>
  </lyric>
  <lyric number="2">
    <syllabic>single</syllabic>
    <text>dark</text>
  </lyric>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :C :octave 4)
  :duration 1
  :type :quarter
  :lyrics ((lyric :number "1" :syllabic :single :text "Day")
           (lyric :number "2" :syllabic :single :text "Night")))
(note
  :pitch (pitch :step :E :octave 4)
  :duration 1
  :type :quarter
  :lyrics ((lyric :number "1" :syllabic :single :text "light")
           (lyric :number "2" :syllabic :single :text "dark")))
```

---

### Example 5: Elision (Two Syllables on One Note)

**Musical meaning:** "heav'n" sung as one syllable

**MusicXML:**
```xml
<note>
  <pitch><step>G</step><octave>4</octave></pitch>
  <duration>1</duration>
  <type>quarter</type>
  <lyric number="1">
    <syllabic>single</syllabic>
    <text>heav</text>
    <elision>'</elision>
    <text>n</text>
  </lyric>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :G :octave 4)
  :duration 1
  :type :quarter
  :lyrics ((lyric :number "1"
             :syllabic :single
             :text "heav"
             :extensions ((elision :value "'")
                          (text "n")))))
```

---

### Example 6: Laughing/Humming Voice

**Musical meaning:** Laughing sound on a note

**MusicXML:**
```xml
<note>
  <pitch><step>C</step><octave>5</octave></pitch>
  <duration>1</duration>
  <type>quarter</type>
  <lyric number="1">
    <laughing/>
  </lyric>
</note>

<note>
  <pitch><step>G</step><octave>4</octave></pitch>
  <duration>2</duration>
  <type>half</type>
  <lyric number="1">
    <humming/>
  </lyric>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :C :octave 5)
  :duration 1
  :type :quarter
  :lyrics ((lyric :number "1" :laughing t)))

(note
  :pitch (pitch :step :G :octave 4)
  :duration 2
  :type :half
  :lyrics ((lyric :number "1" :humming t)))
```

---

### Example 7: Named Lyrics (Verse and Chorus)

**Musical meaning:** Different sections identified by name rather than number

**MusicXML:**
```xml
<note>
  <pitch><step>C</step><octave>4</octave></pitch>
  <duration>1</duration>
  <type>quarter</type>
  <lyric name="verse" number="1">
    <syllabic>single</syllabic>
    <text>First</text>
  </lyric>
  <lyric name="chorus">
    <syllabic>single</syllabic>
    <text>All</text>
  </lyric>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :C :octave 4)
  :duration 1
  :type :quarter
  :lyrics ((lyric :name "verse" :number "1" :syllabic :single :text "First")
           (lyric :name "chorus" :syllabic :single :text "All")))
```

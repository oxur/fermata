# Reviews Directory

This directory will contain the review documents generated during Claude Chat sessions:

- `review-chunk-01.md` through `review-chunk-10.md` — From Phase 3 (Chunk Reviews)
- `assembly-report.md` — From Phase 4 (Assembly)

## Expected Files After Phase 3

```
reviews/
├── review-chunk-01.md
├── review-chunk-02.md
├── review-chunk-03.md
├── review-chunk-04.md
├── review-chunk-05.md
├── review-chunk-06.md
├── review-chunk-07.md
├── review-chunk-08.md
├── review-chunk-09.md
└── review-chunk-10.md
```

## Expected Files After Phase 4

```
reviews/
├── review-chunk-01.md
├── ...
├── review-chunk-10.md
└── assembly-report.md
```

## Workflow

1. After Claude Code produces a chunk file, start a new Claude Chat session
2. Use the template from `0003-prompt-chat-chunk-review.md`
3. Paste the chunk content into the template
4. Save the review output here as `review-chunk-NN.md`
5. Repeat for all chunks
6. Use `0004-prompt-chat-assembly.md` for the final assembly

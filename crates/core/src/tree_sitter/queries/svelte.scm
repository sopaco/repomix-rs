; Svelte compress query - extract script block raw content
; Svelte parser treats <script> content as raw_text, so we capture
; the entire script element to preserve the JS/TS definitions within.
(script_element) @script

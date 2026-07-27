<script lang="ts">
  import type { PrefixEntry } from '../types';

  let { prefix = $bindable() }: { prefix: PrefixEntry } = $props();
</script>

<label for="text_sv">Text</label>
<input
  name="text_sv"
  type="text"
  class="editing-text-input"
  bind:value={prefix.translations.sv.text}
/>
<input
  name="text_en"
  type="text"
  class="editing-text-input"
  bind:value={prefix.translations.en.text}
/>
<label for="group_text_sv">Group text</label>
<input
  name="group_text_sv"
  type="text"
  class="editing-text-input"
  bind:value={prefix.translations.sv.group_text}
/>
<input
  name="group_text_en"
  type="text"
  class="editing-text-input"
  bind:value={prefix.translations.en.group_text}
/>

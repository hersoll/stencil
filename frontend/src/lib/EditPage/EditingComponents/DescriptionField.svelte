<script lang="ts">
  let { entry } = $props();
</script>

<label for="desc_sv">Description</label>
<input
  name="desc_sv"
  type="text"
  class="editing-text-input"
  bind:value={entry.desc.sv}
/>
<input
  name="desc_en"
  type="text"
  class="editing-text-input"
  bind:value={entry.desc.en}
/>

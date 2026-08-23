<script lang="ts">
  import { error, releases } from '$src/globalStates.svelte';
  import { API_URL } from '$src/main';
  import showdown from 'showdown';
  import GitHubIcon from '../SVGIcons/GitHubIcon.svelte';

  async function getReleaseNotes() {
    if (releases.notes.length == 0) {
      const response: Response = await fetch(`${API_URL}/releases`);

      if (!response.ok) {
        let text = await response.text();
        error.message = `Status: ${response.status} \n${text}`;
        return;
      }

      const notes: { tag_name: string; created_at: string; body: string }[] =
        await response.json();
      const converter = new showdown.Converter();
      releases.notes = notes.map(note => ({
        tag_name: note.tag_name,
        created_at: note.created_at.split('T')[0],
        body: converter.makeHtml(note.body)
      }));
    }
  }
</script>

<button
  class="version-number"
  popovertarget="release-notes"
  onclick={getReleaseNotes}>{releases.latest_tag}</button
>
<div popover id="release-notes" class="release-notes">
  <a href="https://github.com/hersoll/stencil" target="_blank">
    GitHub <GitHubIcon />
  </a>
  {#each releases.notes as release}
    <div class="release">
      <h1>{release.tag_name}</h1>
      <p>{release.created_at}</p>
      {@html release.body}
    </div>
  {/each}
</div>

<style>
  .version-number {
    background: none;
    margin: 0;
    padding: 0;
    color: var(--primary-text);
    font-size: 0.9rem;
    font-weight: 600;
    position: fixed;
    top: 0.5rem;
    right: 0.75rem;
  }

  .release-notes {
    position: relative;
    margin: auto;
    width: 90%;
    height: 90%;
    max-width: 50rem;
    max-height: 70rem;

    border-radius: 1rem;
    border-color: var(--strong-border);
    background-color: var(--bg-light);
    padding: 0 2rem;
  }

  .release {
    padding: 1.5rem 0;
    border-bottom: 1px solid var(--strong-border);
    h1 {
      color: var(--primary-text);
      padding: 0;
      margin: 0;
    }
    p {
      font-size: 0.9rem;
      color: var(--text-muted);
      padding-bottom: 0.5rem;
    }
  }

  a {
    position: absolute;
    top: 1.5rem;
    right: 1rem;
    display: flex;
    align-items: center;
    gap: 0.4rem;
    text-decoration: none;
  }

  ::backdrop {
    backdrop-filter: blur(3px);
  }

  /* Mobile layout*/
  @container body (width < 50rem) {
    .version-number {
      display: none;
    }
  }
</style>

<script>
  import Dropzone from "svelte-file-dropzone";

  let files = {
    accepted: [],
  };

  function handleFilesSelect(e) {
    const { acceptedFiles, fileRejections } = e.detail;
    files.accepted = [...files.accepted, ...acceptedFiles];
  }

  function uploadfiles(_e) {
    console.log(files.accepted);
    for (let index in files.accepted) {
      let file = files.accepted[index];
      // upload
    }
  }
</script>

<main>
  <h1>sar-waypoints!</h1>
  <p>
    Use this tool to upload GeoJSON from SARTopo to extract waypoints from
    assignment areas
  </p>
  <button on:click={uploadfiles} style="button">Convert</button>

  <Dropzone accept="application/json" on:drop={handleFilesSelect} />
  <ol>
    {#each files.accepted as item}
      <li>
        <div>
          <h2>{item.name}</h2>
          {#await item.text() then text}
            <p>{text}</p>
          {/await}
        </div>
      </li>
    {/each}
  </ol>
</main>

<style>
</style>

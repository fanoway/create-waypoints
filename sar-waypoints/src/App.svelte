<script>
  import Dropzone from "svelte-file-dropzone";

  let files = {
    accepted: [],
  };

  function handleFilesSelect(e) {
    const { acceptedFiles, fileRejections } = e.detail;
    files.accepted = [...files.accepted, ...acceptedFiles];
  }

  async function uploadfiles(_e) {
    console.log(files.accepted);
    for (let index in files.accepted) {
      let file = files.accepted[index];
      const res = await fetch(
        "https://create_waypoints.goldilocks.workers.dev/",
        {
          method: "POST",
          body: file,
          headers: {
            "Access-Control-Allow-Origin": "*",
            "content-type": "application/JSON",
            "Access-Control-Request-Method": "POST",
          },
        }
      );

      const json = await res.json();
      console.log(json);
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
        <h3>{item.name}</h3>
      </li>
    {/each}
  </ol>
</main>

<style>
</style>

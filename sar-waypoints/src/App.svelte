<script>
  import Dropzone from "svelte-file-dropzone";
  import GeoJsonToGpx from "@dwayneparton/geojson-to-gpx";

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

      const geoJSON = await res.json();
      // console.log(geoJSON);

      let file_name_no_ext = file.name.replace(/\.[^/.]+$/, "");
      const options = {
        metadata: {
          name: file_name_no_ext,
          author: {
            name: "sar-waypoints",
          },
        },
      };
      const gpx = GeoJsonToGpx(geoJSON, options);
      // console.log(gpx);

      // convert document to string or post process it
      let gpxString = new XMLSerializer().serializeToString(gpx);

      // Change creator string
      gpxString = gpxString.replace(
        'creator="@dwayneparton/geojson-to-gpx"',
        'creator="sar.ardron.one and @dwayneparton/geojson-to-gpx"'
      );

      // @see https://stackoverflow.com/questions/10654971/create-text-file-from-string-using-js-and-html5
      const link = document.createElement("a");
      link.download = file_name_no_ext + ".gpx";
      const blob = new Blob([gpxString], { type: "text/xml" });
      link.href = window.URL.createObjectURL(blob);
      link.click();
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

<script lang="ts">
import { onMount } from "svelte"
import "./app.css"
import { invoke } from "@tauri-apps/api/core"

onMount(() => {
	const supportedConstraints = navigator.mediaDevices.getSupportedConstraints()
	const devices = navigator.mediaDevices.enumerateDevices() // Finally, i found the sauce 😭
	console.log(devices.then(console.log))
	const stream = navigator.mediaDevices.getUserMedia({
		audio: { deviceId: "" },
	})
})
</script>

<main>
  <p class="text-3xl">dafsfsfd</p></main>

<script lang="ts">
  import { onMount } from "svelte"
  import "./app.css"
  import { invoke } from "@tauri-apps/api/core"
  import AudioTrack from "../components/AudioTrack.svelte"

  onMount(() => {
    const supportedConstraints =
      navigator.mediaDevices.getSupportedConstraints()
    const devices = navigator.mediaDevices.enumerateDevices() // Finally, i found the sauce 😭
    console.log(devices.then(console.log))
    const stream = navigator.mediaDevices.getUserMedia({
      audio: { deviceId: "" },
    })
  })
</script>

<main class="w-full h-full px-6 py-3 flex flex-col gap-y-4">
  <section>
    <p class="text-sm font-medium">Video</p>
    <img src="/asdf.png" alt="" class="mt-2" />
  </section>

  <section class="flex flex-row items-start gap-x-6">
    <div class="flex-1">
      <p class="text-sm font-medium">Audio Trakcs</p>
      <div class="bg-neutral-800 rounded-lg p-6 mt-2 flex flex-col gap-y-4">
        <AudioTrack />
      </div>
    </div>
    <aside class="min-w-48">
      <p class="text-sm font-medium">Controls</p>
    </aside>
  </section>
</main>

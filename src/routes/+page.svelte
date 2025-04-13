<script lang="ts">
  import { onMount } from "svelte"
  import { invoke } from "@tauri-apps/api/core"

  import AudioTrack from "../components/AudioTrack.svelte"
  import Button from "../components/ui/Button.svelte"
  import { Plus } from "lucide-svelte"

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

<main class="flex h-lvh max-w-7xl flex-col gap-y-4 px-6 py-3">
  <section>
    <p class="text-sm font-medium">Video</p>
    <img src="/asdf.png" alt="" class="mt-2 w-full" />
  </section>

  <section class="flex h-full flex-row items-start justify-between gap-x-4">
    <div class="flex h-full flex-1 flex-col">
      <p class="text-sm font-medium">Audio Trakcs</p>
      <div
        class="mt-2 flex h-full flex-col gap-y-4 rounded-lg bg-neutral-800 px-5 py-3"
      >
        <AudioTrack />
        <Button class="py-1"><Plus class="size-4" /></Button>
      </div>
    </div>
    <aside class="flex h-full min-w-52 flex-col">
      <p class="text-sm font-medium">Controls</p>
      <div
        class="mt-2 flex h-full flex-col gap-y-4 rounded-lg bg-neutral-800 px-5 py-3"
      ></div>
    </aside>
  </section>
</main>

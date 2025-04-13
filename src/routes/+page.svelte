<script lang="ts">
  import { onMount } from "svelte"

  import AudioTrack from "../components/AudioTrack.svelte"
  import Button from "../components/ui/Button.svelte"
  import { Plus } from "lucide-svelte"
  import { devices } from "../lib/stores.svelte"

  onMount(() => {
    //const supportedConstraints =
    //  navigator.mediaDevices.getSupportedConstraints()
    // const stream = navigator.mediaDevices.getUserMedia({
    //   audio: { deviceId: "" },
    // })

    navigator.mediaDevices
      .enumerateDevices()
      .then((_devices) => ($devices = _devices))
  })
</script>

<main class="flex h-lvh max-w-7xl flex-col gap-y-4 px-5 py-3">
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
        class="mt-2 flex h-full flex-col gap-y-2 rounded-lg bg-neutral-800 p-2"
      >
        <Button class="w-full py-1 text-sm">Enable Replaying</Button>
        <Button class="w-full py-1 text-sm">Settings</Button>
        <div class="mt-auto">
          <label>
            <p class="text-sm">Keybind for saving replays</p>
            <input
              type="text"
              defaultValue="Ctrl + ;"
              class="form-input mt-1.5 w-full rounded-lg bg-neutral-900 px-2 py-1 text-sm"
            />
          </label>
          <Button class="mt-2 w-full py-1 text-sm">Save Replay for 15s</Button>
        </div>
      </div>
    </aside>
  </section>
</main>

<!-- TODO: add a feature that allows users to change duration of replays -->

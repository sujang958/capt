<script>
  import { Trash2, Volume2 } from "lucide-svelte"
</script>

<div>
  <p class="text-sm text-neutral-200">Track 1</p>
  <div class="mt-2 flex flex-row items-center justify-between gap-x-12">
    <select name="" id="" class="w-full rounded-lg bg-neutral-700 px-2 py-1">
      <option value="asdfasf">asdfasfdsa</option>
    </select>
    <div class="flex flex-row items-center gap-x-3">
      <!-- svelte-ignore a11y_consider_explicit_label -->
      <button class="cursor-pointer rounded-lg bg-neutral-700 p-2" type="button"
        ><Volume2 class="size-4" />
      </button>
      <button class="cursor-pointer rounded-lg bg-red-300 p-2" type="button"
        ><Trash2 class="size-4 stroke-red-900" />
      </button>
    </div>
  </div>
</div>

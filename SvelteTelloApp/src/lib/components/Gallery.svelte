<script lang="ts">
  import { onMount } from 'svelte';
  import { galleryStore } from '$lib/stores/gallery';
  import Card from './ui/card/Card.svelte';
  import CardHeader from './ui/card/CardHeader.svelte';
  import CardTitle from './ui/card/CardTitle.svelte';
  import CardContent from './ui/card/CardContent.svelte';
  import Button from './ui/button/Button.svelte';
  import { Image, Trash2, Download, FolderOpen } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { toast } from 'svelte-sonner';
  
  let images = [];
  
  onMount(async () => {
    console.log('[Gallery] Loading images...');
    await loadImages();
    
    galleryStore.subscribe($images => {
      images = $images;
    });
  });
  
  async function loadImages() {
    try {
      // TODO: Load images from local storage via Tauri
      console.log('[Gallery] Loading images from storage...');
    } catch (error) {
      console.error('[Gallery] Failed to load images:', error);
    }
  }
  
  async function deleteImage(id: string) {
    if (confirm('Delete this image?')) {
      try {
        console.log('[Gallery] Deleting image:', id);
        // TODO: Delete file via Tauri
        galleryStore.removeImage(id);
        toast.success('Image deleted');
      } catch (error) {
        console.error('[Gallery] Delete failed:', error);
        toast.error('Failed to delete image');
      }
    }
  }
  
  async function openFolder() {
    try {
      console.log('[Gallery] Opening images folder...');
      // TODO: Open folder via Tauri
      toast.info('Opening images folder...');
    } catch (error) {
      console.error('[Gallery] Failed to open folder:', error);
    }
  }
</script>

<Card>
  <CardHeader>
    <div class="flex items-center justify-between">
      <CardTitle class="flex items-center gap-2">
        <Image class="h-5 w-5" />
        Gallery
      </CardTitle>
      <Button on:click={openFolder} variant="ghost" size="sm">
        <FolderOpen class="h-4 w-4 mr-2" />
        Open Folder
      </Button>
    </div>
  </CardHeader>
  <CardContent>
    {#if images.length === 0}
      <div class="text-center py-12">
        <Image class="h-16 w-16 mx-auto mb-4 opacity-30" style="color: var(--color-text-muted)" />
        <p class="text-base font-medium mb-2" style="color: var(--color-text)">No Images Yet</p>
        <p class="text-sm" style="color: var(--color-text-muted)">
          Captured images will appear here
        </p>
      </div>
    {:else}
      <div class="grid grid-cols-2 md:grid-cols-3 gap-4 max-h-96 overflow-y-auto">
        {#each images as image}
          <div class="relative group rounded-lg overflow-hidden border" style="border-color: var(--color-border)">
            <!-- Image -->
            {#if image.thumbnail || image.path}
              <img 
                src={image.thumbnail || image.path} 
                alt={image.filename}
                class="w-full aspect-video object-cover"
                style="background-color: var(--color-surface)"
              />
            {:else}
              <div class="aspect-video flex items-center justify-center" style="background-color: var(--color-surface)">
                <Image class="h-8 w-8 opacity-30" style="color: var(--color-text-muted)" />
              </div>
            {/if}
            
            <!-- Overlay on hover -->
            <div class="absolute inset-0 transition-all flex items-center justify-center gap-2 opacity-0 group-hover:opacity-100" style="background-color: rgba(0,0,0,0.7)">
              <Button variant="ghost" size="icon" on:click={() => deleteImage(image.id)}>
                <Trash2 class="h-5 w-5" style="color: var(--color-error)" />
              </Button>
            </div>
            
            <!-- Info -->
            <div class="p-2" style="background-color: var(--color-surface)">
              <p class="text-xs truncate" style="color: var(--color-text)">{image.filename}</p>
              <p class="text-xs" style="color: var(--color-text-muted)">
                {new Date(image.timestamp).toLocaleDateString()}
              </p>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </CardContent>
</Card>


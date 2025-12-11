<script lang="ts">
  import { onMount } from 'svelte';
  import { modelsStore, recognitionStore } from '$lib/stores/recognition';
  import { droneStore } from '$lib/stores/drone';
  import Card from './ui/card/Card.svelte';
  import CardHeader from './ui/card/CardHeader.svelte';
  import CardTitle from './ui/card/CardTitle.svelte';
  import CardContent from './ui/card/CardContent.svelte';
  import Button from './ui/button/Button.svelte';
  import Input from './ui/input/Input.svelte';
  import Label from './ui/label/Label.svelte';
  import { 
    ScanFace, 
    Play, 
    Square, 
    Trash2, 
    Plus,
    Camera
  } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { toast } from 'svelte-sonner';
  
  let newModelName = '';
  let trainingMode = false;
  let samplesCollected = 0;
  let targetSamples = 20;
  
  onMount(async () => {
    await refreshModels();
  });
  
  async function refreshModels() {
    try {
      const models = await invoke('list_face_models');
      $modelsStore.set(models);
    } catch (error) {
      console.error('Failed to load models:', error);
    }
  }
  
  function startTraining() {
    if (!newModelName.trim()) {
      toast.error('Enter a name for the model');
      return;
    }
    
    if (!$droneStore.videoActive) {
      toast.error('Start video stream first');
      return;
    }
    
    trainingMode = true;
    samplesCollected = 0;
    toast.info(`Training mode started for ${newModelName}`);
  }
  
  function captureSample() {
    samplesCollected++;
    // TODO: Capture and save sample
    
    if (samplesCollected >= targetSamples) {
      finishTraining();
    } else {
      toast.success(`Sample ${samplesCollected}/${targetSamples} captured`);
    }
  }
  
  async function finishTraining() {
    try {
      await invoke('train_face_model', { name: newModelName });
      trainingMode = false;
      samplesCollected = 0;
      newModelName = '';
      await refreshModels();
      toast.success('Model training completed!');
    } catch (error) {
      toast.error('Training failed: ' + error);
    }
  }
  
  function cancelTraining() {
    trainingMode = false;
    samplesCollected = 0;
    newModelName = '';
    toast.info('Training cancelled');
  }
  
  async function deleteModel(name: string) {
    if (confirm(`Delete model "${name}"?`)) {
      try {
        await invoke('delete_face_model', { name });
        await refreshModels();
        toast.success(`Model "${name}" deleted`);
      } catch (error) {
        toast.error('Delete failed: ' + error);
      }
    }
  }
  
  async function startRecognition(modelName: string) {
    if (!$droneStore.videoActive) {
      toast.error('Start video stream first');
      return;
    }
    
    try {
      await invoke('start_face_recognition', { modelName });
      $recognitionStore.start(modelName);
      toast.success(`Recognition started with ${modelName}`);
    } catch (error) {
      toast.error('Failed to start recognition: ' + error);
    }
  }
  
  async function stopRecognition() {
    try {
      await invoke('stop_face_recognition');
      $recognitionStore.stop();
      toast.info('Recognition stopped');
    } catch (error) {
      toast.error('Failed to stop recognition: ' + error);
    }
  }
</script>

<Card>
  <CardHeader>
    <CardTitle class="flex items-center gap-2">
      <ScanFace class="h-5 w-5" />
      Face Recognition
    </CardTitle>
  </CardHeader>
  <CardContent>
    <div class="space-y-6">
      {#if !trainingMode}
        <!-- Model List -->
        <div class="space-y-3">
          <p class="text-sm font-semibold theme-text">Trained Models:</p>
          
          {#if $modelsStore.length === 0}
            <div class="text-center py-4 theme-text-muted">
              <p class="text-sm">No models trained yet</p>
            </div>
          {:else}
            <div class="space-y-2">
              {#each $modelsStore as model}
                <div class="flex items-center justify-between p-3 theme-surface rounded-lg border theme-border">
                  <div>
                    <p class="font-medium theme-text">{model.name}</p>
                    <p class="text-xs theme-text-muted">
                      {model.algorithm.toUpperCase()} • {model.sampleCount} samples
                    </p>
                  </div>
                  <div class="flex gap-2">
                    {#if $recognitionStore.active && $recognitionStore.currentModel === model.name}
                      <Button 
                        on:click={stopRecognition}
                        variant="destructive"
                        size="sm"
                      >
                        <Square class="h-4 w-4" />
                      </Button>
                    {:else}
                      <Button 
                        on:click={() => startRecognition(model.name)}
                        variant="outline"
                        size="sm"
                        disabled={$recognitionStore.active}
                      >
                        <Play class="h-4 w-4" />
                      </Button>
                    {/if}
                    <Button 
                      on:click={() => deleteModel(model.name)}
                      variant="outline"
                      size="sm"
                    >
                      <Trash2 class="h-4 w-4" />
                    </Button>
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        </div>
        
        <!-- New Model -->
        <div class="space-y-3">
          <p class="text-sm font-semibold theme-text">Train New Model:</p>
          <div class="space-y-2">
            <Label for="model-name">Model Name</Label>
            <Input 
              id="model-name"
              bind:value={newModelName}
              placeholder="Enter name..."
            />
          </div>
          <Button 
            on:click={startTraining}
            class="w-full theme-primary"
            disabled={!newModelName.trim() || !$droneStore.videoActive}
          >
            <Plus class="mr-2 h-4 w-4" />
            Start Training
          </Button>
        </div>
      {:else}
        <!-- Training Mode -->
        <div class="space-y-4">
          <div class="p-4 bg-info bg-opacity-10 rounded-lg">
            <p class="font-semibold theme-text mb-2">Training: {newModelName}</p>
            <p class="text-sm theme-text-muted mb-3">
              Samples collected: {samplesCollected}/{targetSamples}
            </p>
            <div class="w-full bg-gray-200 rounded-full h-2">
              <div 
                class="bg-info h-2 rounded-full transition-all"
                style="width: {(samplesCollected / targetSamples) * 100}%"
              />
            </div>
          </div>
          
          <Button 
            on:click={captureSample}
            class="w-full theme-primary"
          >
            <Camera class="mr-2 h-4 w-4" />
            Capture Sample ({samplesCollected}/{targetSamples})
          </Button>
          
          <div class="flex gap-2">
            <Button 
              on:click={finishTraining}
              variant="outline"
              class="flex-1"
              disabled={samplesCollected < 10}
            >
              Finish Training
            </Button>
            <Button 
              on:click={cancelTraining}
              variant="destructive"
              class="flex-1"
            >
              Cancel
            </Button>
          </div>
          
          <p class="text-xs theme-text-muted text-center">
            Position face in frame and click Capture Sample. Move slightly between samples.
          </p>
        </div>
      {/if}
      
      {#if $recognitionStore.active}
        <div class="p-3 bg-success bg-opacity-10 rounded-lg border border-success">
          <p class="text-sm font-semibold text-success mb-1">
            Recognition Active
          </p>
          {#if $recognitionStore.recognizedName}
            <p class="text-sm theme-text">
              Detected: {$recognitionStore.recognizedName} 
              ({Math.round($recognitionStore.confidence * 100)}%)
            </p>
          {:else}
            <p class="text-sm theme-text-muted">Scanning for faces...</p>
          {/if}
        </div>
      {/if}
    </div>
  </CardContent>
</Card>

# Migration Guide: Angular to Svelte

This guide helps you understand the migration from the Angular frontend to the new Svelte + Tauri application.

## Overview

We're migrating from:
- **Angular** (InstallFrontEnd/TelloDroneFrontEnd)
- **Web-based** application

To:
- **Svelte** + **Tauri** (SvelteTelloApp)
- **Desktop** application (cross-platform)

## Why Migrate?

### Advantages of Svelte + Tauri

1. **Native Desktop App**
   - No browser required
   - Better performance
   - System integration
   - Smaller bundle size

2. **Modern Development**
   - Faster build times (Vite vs Angular CLI)
   - Simpler syntax (Svelte vs Angular templates)
   - Better TypeScript integration
   - Hot module replacement

3. **Better UX**
   - Native feel and performance
   - Direct file system access
   - System notifications
   - Offline-first capability

4. **Lighter Weight**
   - Smaller runtime (no zone.js, RxJS overhead)
   - Compiled output vs runtime framework
   - Faster load times

## Component Mapping

### Angular → Svelte Component Equivalents

| Angular Component | Svelte Component | Status |
|------------------|------------------|--------|
| Drone Control Panel | `DroneControl.svelte` | ✅ Created |
| Video Feed | `VideoFeed.svelte` | ✅ Created |
| Face Recognition UI | `FaceRecognition.svelte` | ✅ Created |
| Theme Toggle | `ThemeToggle.svelte` | ✅ Created |
| Settings | `Settings.svelte` | ⏳ Pending |
| Navigation | App-level routing | ⏳ Pending |

## Code Comparison

### Component Structure

**Angular:**
```typescript
@Component({
  selector: 'app-drone-control',
  templateUrl: './drone-control.component.html',
  styleUrls: ['./drone-control.component.css']
})
export class DroneControlComponent implements OnInit {
  connected = false;
  battery = 0;
  
  constructor(private droneService: DroneService) {}
  
  ngOnInit() {
    this.droneService.getStatus().subscribe(status => {
      this.battery = status.battery;
    });
  }
  
  connect() {
    this.droneService.connect();
  }
}
```

**Svelte:**
```typescript
<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  
  let connected = false;
  let battery = 0;
  
  async function connect() {
    await invoke('connect_drone');
    await updateStatus();
  }
  
  async function updateStatus() {
    const status = await invoke('get_drone_status');
    battery = status.battery;
  }
</script>

<button on:click={connect}>Connect</button>
```

### Data Binding

**Angular (Two-way):**
```html
<input [(ngModel)]="personName" />
```

**Svelte (Two-way):**
```html
<input bind:value={personName} />
```

### Conditional Rendering

**Angular:**
```html
<div *ngIf="connected">Connected!</div>
```

**Svelte:**
```html
{#if connected}
  <div>Connected!</div>
{/if}
```

### List Rendering

**Angular:**
```html
<div *ngFor="let item of items">{{ item }}</div>
```

**Svelte:**
```html
{#each items as item}
  <div>{item}</div>
{/each}
```

## Service Layer Migration

### Angular Services → Tauri Commands

**Angular Service:**
```typescript
@Injectable()
export class DroneService {
  connect(): Observable<void> {
    return this.http.post('/api/drone/connect', {});
  }
}
```

**Tauri Command (Rust):**
```rust
#[tauri::command]
async fn connect_drone() -> Result<(), String> {
    // Implementation
    Ok(())
}
```

**Svelte Usage:**
```typescript
import { invoke } from '@tauri-apps/api/tauri';

await invoke('connect_drone');
```

## State Management

### Angular (Services + RxJS)

```typescript
@Injectable()
export class DroneStateService {
  private stateSubject = new BehaviorSubject<DroneState>({});
  state$ = this.stateSubject.asObservable();
  
  updateState(state: DroneState) {
    this.stateSubject.next(state);
  }
}
```

### Svelte (Stores)

```typescript
// store.ts
import { writable } from 'svelte/store';

export const droneState = writable({
  connected: false,
  battery: 0
});

// Component usage
import { droneState } from './store';

$droneState.connected = true;
```

## Styling Migration

### Angular (Component Styles)

```css
/* drone-control.component.css */
.control-panel {
  display: flex;
  flex-direction: column;
}
```

### Svelte (Component Styles + Tailwind)

```svelte
<div class="flex flex-col">
  <!-- content -->
</div>

<style>
  /* Component-scoped styles if needed */
</style>
```

## HTTP to IPC Migration

### Angular (HTTP Client)

```typescript
this.http.post('/api/drone/command', { cmd: 'takeoff' })
  .subscribe(result => {
    console.log('Command sent');
  });
```

### Svelte + Tauri (IPC)

```typescript
await invoke('send_drone_command', { command: 'takeoff' });
console.log('Command sent');
```

## Routing Migration

### Angular Router

```typescript
const routes: Routes = [
  { path: 'control', component: DroneControlComponent },
  { path: 'settings', component: SettingsComponent }
];
```

### Svelte (Options)

**Option 1: Single Page (Current)**
- Use tabs or panels
- No routing needed for simple apps

**Option 2: SvelteKit Router**
- Migrate to SvelteKit
- File-based routing

**Option 3: svelte-spa-router**
```typescript
import Router from 'svelte-spa-router';

const routes = {
  '/control': DroneControl,
  '/settings': Settings
};
```

## Dependency Injection

### Angular

```typescript
constructor(
  private drone: DroneService,
  private video: VideoService
) {}
```

### Svelte

**No DI needed** - Direct imports:
```typescript
import { connectDrone } from '$lib/services/drone';
```

Or use Svelte context for component trees:
```typescript
// Parent
setContext('drone', droneService);

// Child
const drone = getContext('drone');
```

## Forms

### Angular Reactive Forms

```typescript
this.form = this.fb.group({
  personName: ['', Validators.required]
});
```

### Svelte (Simple)

```svelte
<script>
  let personName = '';
  let errors = {};
  
  function validate() {
    errors = {};
    if (!personName) {
      errors.personName = 'Required';
    }
  }
</script>

<input bind:value={personName} />
{#if errors.personName}
  <span class="error">{errors.personName}</span>
{/if}
```

Or use libraries like:
- `svelte-forms-lib`
- `felte`

## Testing

### Angular (Jasmine + Karma)

```typescript
describe('DroneControlComponent', () => {
  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
```

### Svelte (Vitest + Testing Library)

```typescript
import { render, fireEvent } from '@testing-library/svelte';
import DroneControl from './DroneControl.svelte';

test('renders component', () => {
  const { getByText } = render(DroneControl);
  expect(getByText('Connect')).toBeInTheDocument();
});
```

## Migration Checklist

### Phase 1: Setup ✅
- [x] Create Svelte project structure
- [x] Set up Tauri
- [x] Configure Tailwind CSS
- [x] Set up shadcn-ui components
- [x] Configure themes

### Phase 2: Core Features (In Progress)
- [x] Create drone control UI
- [x] Create video feed component
- [x] Create face recognition UI
- [ ] Implement Tauri commands
- [ ] Connect to Python backend
- [ ] Implement video streaming

### Phase 3: Advanced Features
- [ ] Add settings page
- [ ] Implement keyboard shortcuts
- [ ] Add flight path recording
- [ ] Add error handling
- [ ] Add notifications

### Phase 4: Testing & Polish
- [ ] Write component tests
- [ ] Write integration tests
- [ ] Performance optimization
- [ ] UI/UX improvements
- [ ] Documentation

### Phase 5: Deployment
- [ ] Build for all platforms
- [ ] Create installers
- [ ] Set up auto-updates
- [ ] Release v1.0

## Breaking Changes

### API Changes
- HTTP endpoints → Tauri commands
- WebSocket → Tauri events
- REST API → IPC

### Architecture Changes
- Web app → Desktop app
- Browser-based → Native
- Client-server → Embedded backend

### Development Changes
- Angular CLI → Vite
- npm start → npm run tauri:dev
- ng build → npm run tauri:build

## Tips for Migration

1. **Start Small**: Migrate one component at a time
2. **Test Frequently**: Test each component as you migrate
3. **Use TypeScript**: Maintain type safety
4. **Leverage Tauri**: Use native features where beneficial
5. **Keep Python Backend**: No need to rewrite drone logic

## Common Pitfalls

1. **Async Operations**: Remember to use `await` with Tauri commands
2. **Reactivity**: Svelte requires assignments for reactivity (`items = [...items, newItem]`)
3. **Component Lifecycle**: Different from Angular (no ngOnInit, use onMount)
4. **Styling Scope**: Svelte styles are scoped by default
5. **File Structure**: Flat structure works well, no need for folders per component

## Resources

- [Svelte Tutorial](https://svelte.dev/tutorial)
- [Tauri Guides](https://tauri.app/v1/guides/)
- [Svelte vs Angular](https://svelte.dev/blog/frameworks-without-the-framework)
- [Tailwind CSS](https://tailwindcss.com/docs)
- [shadcn-svelte](https://www.shadcn-svelte.com/)

## Getting Help

1. Check Svelte REPL for syntax questions
2. Review Tauri examples
3. Check project documentation in `docs/`
4. Open GitHub issues for specific problems

---

**Next**: See [PROJECT_STATUS.md](PROJECT_STATUS.md) for current progress


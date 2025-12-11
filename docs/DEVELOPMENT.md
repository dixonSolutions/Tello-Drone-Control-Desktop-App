# Development Guide

## Getting Started

### Prerequisites
1. **Node.js and npm**
   ```bash
   node --version  # Should be 18+
   npm --version
   ```

2. **Rust**
   ```bash
   rustc --version  # Should be 1.70+
   cargo --version
   ```
   
   If not installed:
   ```bash
   # Windows
   winget install Rustlang.Rustup
   
   # macOS/Linux
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **Tauri Prerequisites**
   - Windows: Microsoft Visual Studio C++ Build Tools
   - macOS: Xcode Command Line Tools
   - Linux: See [Tauri docs](https://tauri.app/v1/guides/getting-started/prerequisites)

### First Time Setup

1. Clone and navigate:
```bash
cd SvelteTelloApp
```

2. Install JavaScript dependencies:
```bash
npm install
```

3. Start development server:
```bash
npm run tauri:dev
```

The first build will take 5-10 minutes as Rust compiles all dependencies.

## Development Workflow

### Running the App
```bash
npm run tauri:dev
```

This starts:
- Vite dev server (hot reload for Svelte changes)
- Tauri development window
- Rust compilation (when needed)

### Code Structure

#### Frontend (`src/`)
- `App.svelte` - Main app layout and routing
- `lib/components/` - Reusable UI components
- `lib/stores/` - Svelte stores for state management
- `lib/types/` - TypeScript type definitions
- `lib/themes.ts` - Theme system
- `lib/api.ts` - Tauri API wrapper

#### Backend (`src-tauri/`)
- `src/main.rs` - All Tauri commands and drone communication
- `Cargo.toml` - Rust dependencies
- `tauri.conf.json` - App configuration

### Making Changes

#### Adding a New Component
1. Create file in `src/lib/components/`
2. Use TypeScript and Svelte 4 syntax
3. Import and use in `App.svelte`

Example:
```svelte
<script lang="ts">
  import { droneStore } from '$lib/stores/drone';
  import Card from './ui/card/Card.svelte';
  
  // Component logic
</script>

<Card>
  <!-- UI markup -->
</Card>
```

#### Adding a New Tauri Command

1. Add command function in `src-tauri/src/main.rs`:
```rust
#[tauri::command]
async fn my_command(param: String) -> Result<String, String> {
    Ok(format!("Received: {}", param))
}
```

2. Register in `invoke_handler!`:
```rust
.invoke_handler(tauri::generate_handler![
    my_command,
    // ... other commands
])
```

3. Call from frontend:
```typescript
import { invoke } from '@tauri-apps/api/tauri';
const result = await invoke('my_command', { param: 'value' });
```

#### Modifying Stores

Stores are in `src/lib/stores/`. Example:

```typescript
import { writable } from 'svelte/store';

function createMyStore() {
  const { subscribe, set, update } = writable(initialValue);
  
  return {
    subscribe,
    myAction: () => update(state => ({ ...state, changed: true })),
  };
}

export const myStore = createMyStore();
```

#### Adding a Theme

Edit `src/lib/themes.ts`:

```typescript
mytheme: {
  name: 'mytheme',
  displayName: 'My Theme',
  primary: '#3c6eb4',
  // ... other colors
},
```

### Testing Drone Connection

1. Turn on Tello drone
2. Connect computer to Tello WiFi (TELLO-XXXXXX)
3. Run app and click "Connect"
4. Check console for debug messages

### Debugging

#### Frontend Debugging
- Open DevTools in the Tauri window: Right-click → Inspect Element
- Console logs appear in DevTools
- Use `console.log()`, `console.error()`, etc.

#### Backend Debugging
- Rust `println!()` appears in terminal where you ran `npm run tauri:dev`
- Use `dbg!()` macro for debug output:
```rust
dbg!(&some_variable);
```

### Hot Reload

- **Frontend changes**: Auto-reload (Vite HMR)
- **Rust changes**: Requires rebuild (kills and restarts app)

## Building for Production

### Development Build
```bash
npm run tauri:build
```

Outputs to `src-tauri/target/release/bundle/`

### Release Build
```bash
npm run tauri:build -- --release
```

Creates optimized installer/executable.

## Common Tasks

### Adding a New Icon
1. Add to `lucide-svelte` imports:
```typescript
import { MyIcon } from 'lucide-svelte';
```

2. Use in component:
```svelte
<MyIcon class="h-4 w-4" />
```

### Styling with Theme Colors
Use CSS custom properties:

```svelte
<div class="theme-surface theme-border">
  <p class="theme-text">Text</p>
  <p class="theme-text-muted">Muted text</p>
</div>
```

Or use Tailwind utilities:
```svelte
<div class="bg-success text-white">Success!</div>
```

### Adding Settings
1. Add to `settingsStore` in `src/lib/stores/settings.ts`
2. Add UI in `src/lib/components/Settings.svelte`
3. Use in components via `$settingsStore.myNewSetting`

### Error Handling
Use toast notifications:

```typescript
import { toast } from 'svelte-sonner';

try {
  await someAsyncOperation();
  toast.success('Operation successful');
} catch (error) {
  toast.error('Operation failed: ' + error);
}
```

## Performance Tips

1. **Avoid unnecessary re-renders**: Use `$:` reactive statements carefully
2. **Debounce expensive operations**: Like video frame processing
3. **Use async/await**: For all Tauri commands
4. **Minimize store subscriptions**: Unsubscribe when component unmounts

## Code Style

- **TypeScript**: Use explicit types
- **Svelte**: Use `lang="ts"` in script tags
- **Formatting**: Use Prettier (auto-format on save)
- **Naming**: camelCase for JS, PascalCase for components
- **Comments**: Explain "why" not "what"

## Troubleshooting

### Rust won't compile
```bash
# Clean build
cd src-tauri
cargo clean
cd ..
npm run tauri:dev
```

### Frontend errors
```bash
# Clear cache
rm -rf node_modules
npm install
```

### Tauri command not found
- Check command is registered in `invoke_handler!`
- Verify function has `#[tauri::command]` attribute
- Ensure function is async if it returns `Result`

### Port already in use
Vite default is 5173. Change in `vite.config.ts`:
```typescript
export default defineConfig({
  server: { port: 5174 },
  // ...
});
```

## Resources

- [Svelte Tutorial](https://svelte.dev/tutorial)
- [Tauri Documentation](https://tauri.app/v1/guides/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tello SDK](https://dl-cdn.ryzerobotics.com/downloads/Tello/Tello%20SDK%202.0%20User%20Guide.pdf)

## Need Help?

- Check existing issues on GitHub
- Read the Tello SDK documentation
- Review Python implementation for reference
- Ask in discussions
import { Injectable, signal } from '@angular/core';

@Injectable({ providedIn: 'root' })
export class Datasvr {
  private static readonly STORAGE_KEY = 'telloapp.darkMode';
  readonly darkMode = signal<boolean>(true);

  constructor() {
    if (this.isBrowser()) {
      try {
        const raw = localStorage.getItem(Datasvr.STORAGE_KEY);
        if (raw === 'false') this.darkMode.set(false);
      } catch {}
      this.applyToDom(this.darkMode());
    }
  }

  toggleDarkMode(): void {
    const next = !this.darkMode();
    this.setDarkMode(next);
  }

  setDarkMode(val: boolean): void {
    this.darkMode.set(val);
    if (this.isBrowser()) {
      try { localStorage.setItem(Datasvr.STORAGE_KEY, String(val)); } catch {}
      this.applyToDom(val);
    }
  }

  private applyToDom(isDark: boolean): void {
    if (!this.isBrowser()) return;
    document.documentElement.classList.toggle('app-dark', isDark);
  }

  private isBrowser(): boolean {
    return typeof window !== 'undefined' && typeof document !== 'undefined';
  }
}



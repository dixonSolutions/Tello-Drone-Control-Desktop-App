import { Component, EventEmitter, Output, OnInit, signal } from '@angular/core';
import { MenubarModule } from 'primeng/menubar';
import { ButtonModule } from 'primeng/button';
import { ToggleSwitch } from 'primeng/toggleswitch';
import { NgClass } from '@angular/common';
import { Router, RouterLink, RouterLinkActive } from '@angular/router';
import { FormsModule } from '@angular/forms';
import { Sidebar } from '../sidebar/sidebar';
import { Datasvr } from '../datasvr';

@Component({
  selector: 'app-topbar',
  standalone: true,
  imports: [MenubarModule, ButtonModule, ToggleSwitch, FormsModule, NgClass, RouterLink, RouterLinkActive, Sidebar],
  templateUrl: './topbar.html',
  styleUrl: './topbar.css',
})
export class Topbar implements OnInit {
  @Output() menuToggle = new EventEmitter<void>();
  protected readonly drawerVisible = signal(false);

  constructor(private router: Router, protected data: Datasvr) {}

  ngOnInit(): void {
    // Datasvr constructor applies the stored theme to DOM
  }

  onToggleMenu(): void {
    this.drawerVisible.set(!this.drawerVisible());
    this.menuToggle.emit();
  }

  onToggleTheme(): void { this.data.toggleDarkMode(); }

  goToFeatures(): void {
    this.router.navigate(['/'], { fragment: 'features' });
  }
}

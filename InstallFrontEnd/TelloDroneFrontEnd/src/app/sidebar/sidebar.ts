import { Component, EventEmitter, Input, Output } from '@angular/core';
import { Drawer } from 'primeng/drawer';
import { PanelMenu } from 'primeng/panelmenu';
import { MenuItem } from 'primeng/api';

@Component({
  selector: 'app-sidebar',
  standalone: true,
  imports: [Drawer, PanelMenu],
  templateUrl: './sidebar.html',
  styleUrl: './sidebar.css',
})
export class Sidebar {
  @Input() visible = false;
  @Output() visibleChange = new EventEmitter<boolean>();

  items: MenuItem[] = [
    {
      label: 'Home',
      icon: 'pi pi-home',
      routerLink: ['/']
    },
    {
      label: 'Install',
      icon: 'pi pi-download',
      routerLink: ['/install']
    }
  ];

  onHide(): void {
    this.visibleChange.emit(false);
  }
}

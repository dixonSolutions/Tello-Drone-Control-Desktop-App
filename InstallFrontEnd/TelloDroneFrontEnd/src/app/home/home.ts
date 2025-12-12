import { Component } from '@angular/core';
import { Button } from 'primeng/button';
import { Divider } from 'primeng/divider';
import { Topbar } from '../topbar/topbar';
import { Sidebar } from '../sidebar/sidebar';
import { Router, RouterModule } from '@angular/router';
@Component({
  selector: 'app-home',
  standalone: true,
  imports: [Button, Divider, Topbar, Sidebar, RouterModule],
  templateUrl: './home.html',
  styleUrl: './home.css',
})
export class Home {
  constructor(private router: Router) {}
  onInstall(): void {
    this.router.navigate(['/install']);
  }

}

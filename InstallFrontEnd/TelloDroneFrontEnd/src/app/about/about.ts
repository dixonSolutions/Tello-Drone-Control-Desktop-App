import { Component } from '@angular/core';
import { Topbar } from '../topbar/topbar';

@Component({
  selector: 'app-about',
  standalone: true,
  imports: [Topbar],
  templateUrl: './about.html',
  styleUrl: './about.css',
})
export class About {}



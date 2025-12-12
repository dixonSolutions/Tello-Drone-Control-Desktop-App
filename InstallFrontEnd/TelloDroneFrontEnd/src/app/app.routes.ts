import { Routes } from '@angular/router';
import { Home } from './home/home';
import { Install } from './install/install';
import { About } from './about/about';

export const routes: Routes = [
  { path: '', component: Home },
  { path: 'install', component: Install },
  { path: 'about', component: About },
  { path: '**', component: Home }
];

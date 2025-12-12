import { Component } from '@angular/core';
import { Topbar } from '../topbar/topbar';
import { AccordionModule } from 'primeng/accordion';
import { ButtonModule } from 'primeng/button';
import { Divider } from 'primeng/divider';
import { Router} from '@angular/router';
import { RouterModule } from '@angular/router';
@Component({
  selector: 'app-install',
  standalone: true,
  imports: [Topbar, AccordionModule, ButtonModule, Divider],
  templateUrl: './install.html',
  styleUrl: './install.css',
})
export class Install {
  constructor( public hello: Router){

  }
  install_fedora(){
    

  }

}

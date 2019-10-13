define(function (require) {
  let Vue = require('vue');
  let Store = require('store/appstore')
  
  Vue.component('Menu', {
    data:  function () {
      return { 
        state: Store.state
      }
    },
    methods: {
      patchSelected(s){
        Store.selectPatch(s);
      }
    },
    template: `
    <div>
    <b-navbar toggleable="lg" type="dark" >
      <b-navbar-brand href="#">Monoxyd Synth</b-navbar-brand>
  
      <b-navbar-toggle target="nav-collapse"></b-navbar-toggle>
  
      <b-collapse id="nav-collapse" is-nav>
  
        <b-navbar-nav class="ml-auto">

          <b-nav-item-dropdown text="Patch" right>
            <b-dropdown-item v-for="item in state.patches" 
              href="#" 
              :key="item"
              @click="patchSelected(item)">
              {{item}}
            </b-dropdown-item>
          </b-nav-item-dropdown>
  
        </b-navbar-nav>
      </b-collapse>
    </b-navbar>
  </div>`,
  });
});
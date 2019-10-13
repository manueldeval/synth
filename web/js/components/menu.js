define(function (require) {
  let Vue = require('vue');
  let Store = require('store/appstore')
  
  Vue.component('Menu', {
    data:  function () {
      return { 
        state: Store.state,
        newPatch: ''
      }
    },
    methods: {
      patchSelected(s){
        Store.selectPatch(s);
      },
      savePatch(){
        if (this.state.current_patch == "New"){
          this.$refs['modal'].show()
        } else {
          Store.savePatch();
        }
      },
      handleOk(){
        if (this.newPatch != null && 
          this.newPatch != '' && 
          this.newPatch !="New") {
            this.state.current_patch = this.newPatch;
            this.savePatch();
        }
      }
    },
    template: `
    <div>
    <b-navbar toggleable="lg" type="dark" >
      <b-navbar-brand href="#">Monoxyd Synth</b-navbar-brand>
  
      <b-navbar-toggle target="nav-collapse"></b-navbar-toggle>
  
      <b-collapse id="nav-collapse" is-nav>
  
        <b-navbar-nav class="ml-auto">

          <b-nav-form>
            <b-button 
              v-if="state.dirty"
              @click="savePatch()"
              size="sm" class="my-2 my-sm-0" >
              Save
            </b-button>
          </b-nav-form>
  
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

    <b-modal ref="modal" id="modal-sm" size="sm" title="Enter a patch name" @ok="handleOk">
      <b-form-input id="patch_id"
                    v-model="newPatch"
                    required>
      </b-form-input>
    </b-modal>

  </div>`,
  });
});
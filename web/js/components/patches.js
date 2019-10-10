define(function (require) {
  let Vue = require('vue');
  let Store = require('store/appstore')

  Vue.component('Patches', {
    data:  function () {
      return { 
        patches: Store.state.patches
      }
    },
    template: `<div>
    <ul id="example-1">
      <li v-for="item in patches">
        {{ item }}
      </li>
    </ul>
    </div>`,
  
    });
});
define(function (require) {
  let Vue = require('vue');
  let Store = require('store/appstore')

  Vue.component('Menu', {
    data:  function () {
      return { 
        state: Store.state
      }
    },
    template: `
    <div>
      <ul id="example-1">
        <li v-for="item in state.nodeTypes">
          {{ item.type }}
        </li>
      </ul>
    </div>`,
  });
});
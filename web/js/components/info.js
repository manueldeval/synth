define(function (require) {
  let Vue = require('vue');
  let Store = require('store/appstore')
  
  Vue.component('Info', {
    data:  function () {
      return { 
        state: Store.state
      }
    },
    template: `
    <div style="height:30px;background-color: #666666;color:white; padding-left: 5px;font-size: 0.9em;">
    Patch: {{ state.current_patch }}
    </div>`,
  });
});
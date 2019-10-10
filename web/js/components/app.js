define(function (require) {
  let Vue = require('vue');

  require('components/patches');
  require('components/graph');
  require('components/menu');

  Vue.component('App', {
    template: `<div>
      <Menu></Menu>
        <b-tabs>
          <b-tab title="Editor" active >
          <Graph></Graph>
          </b-tab>
          <b-tab title="Patches">
          <Patches></Patches>
          </b-tab>
        </b-tabs>
    </div>`,
    });
});
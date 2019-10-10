define(function (require) {
  let Vue = require('vue');

  require('components/patches');
  require('components/graph');
  require('components/menu');

  Vue.component('App', {
    template: `<div>
      <Menu></Menu>
      <div style="height:100%;">
      <b-card no-body>
        <b-tabs card>
          <b-tab title="Patches" active >
            <Patches></Patches>
          </b-tab>
          <b-tab title="Editor">
            <Graph></Graph>
          </b-tab>
        </b-tabs>
      </b-card>


      
      
    </div>`,
    });
});
define(function (require) {
  // Add global scopr graph
  require('litegraph/litegraph');


  let axios = require("axios");
  let graph = new LGraph();

  var Store = {
    state: {
      nodeTypes: []
    },
    getGraph(){
      return graph;
    },
    fetchNodeType() {
      return axios.get('/dspnodes').then((response) => {
        var datas = response.data;
        this.clearNodeTypes();
        this.state.nodeTypes.push(...Object.keys(datas)
          .sort()
          .map(key => {
            return {type:key, ... datas[key]}
          }))
      })
    },
    clearNodeTypes(){
      this.state.nodeTypes.length = 0
    }
  }

  return Store;
})
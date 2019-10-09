define(function (require) {
  let axios = require("axios");

  var Store = {
    state: {
      nodeTypes: []
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
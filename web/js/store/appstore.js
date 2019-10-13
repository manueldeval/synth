define(function (require) {
  // Add global scopr graph
  require('litegraph/litegraph');

  let axios = require("axios");
  let graph = new LGraph();

  function findNodeWithId(nodeId){
    console.log(graph._nodes)
    return graph._nodes.find(node => node.synth_infos.id == nodeId)
  }


  var Store = {
    state: {
      current_patch: "New",
      dirty: false,
      nodeTypes: [],
      patches: []
    },
    getGraph(){
      return graph;
    },
    // NODE TYPES =========
    fetchNodeType() {
      return axios.get('/dspnodes').then((response) => {
        var datas = response.data;
        this.clearNodeTypes();
        this.state.nodeTypes.push(...Object.keys(datas)
          .sort()
          .map(key => { console.log(key);
            return {type:key, ... datas[key]}
          }))
      })
    },
    clearNodeTypes(){
      this.state.nodeTypes.length = 0
    },
    findNodeType(nodeType){
      return this.state.nodeTypes.find(nt => nt.type == nodeType);
    },
    // PATCHES =========
    selectPatch(patch){
      this.state.current_patch = patch
      if (patch == 'New') {
        this.resetSynth();
      } else {
        return axios.get('/patches/'+patch).then((response) => {
          var datas = response.data;
          datas.commands.forEach(c => this.applyResetCommand(c));
          datas.commands.forEach(c => this.applyCreateCommand(c));
          datas.commands.forEach(c => this.applyLinkCommand(c));
          datas.commands.forEach(c => this.applyConfigCommand(c));
        })
      }
    },
    applyResetCommand(command){
      if (command=="Reset"){
        this.resetSynth();
      }
    },
    applyCreateCommand(command){
      if (command.Create) {
        var id = command.Create.id;
        var nodeType = this.findNodeType(command.Create.node_type);
        if (nodeType == null){
          alert("Node type: "+nodeType+" not found.")
        }
        var node = LiteGraph.createNode(nodeType.classifier);
        node.synth_infos.id = id;
        node.pos = [100,100];
        graph.add(node);
      } 
    },
    applyLinkCommand(command){
      if (command.Link) {
        console.log("link",command)
        var {src_node, src_port, dst_node, dst_port} = command.Link;
        var node1 = findNodeWithId(src_node);
        var node2 = findNodeWithId(dst_node);
        node1.connect(src_port, node2, dst_port );
      }
    },
    applyConfigCommand(command){
      if(command.ChangeConfig) {
        var { id, key, val } =  command.ChangeConfig;
        var value = val.StringVal || val.FloatVal || val.IntVal || val.BoolVal;
        var node = findNodeWithId(id);
        node.onPropertyChanged(key,value);
      }
    },
    fetchPatches() {
      return axios.get('/patches').then((response) => {
        var datas = response.data;
        this.clearPatches();
        this.state.patches.push("New", ... datas.sort());
      })
    },
    clearPatches(){
      this.state.patches.length = 0
    },
    sendCommand(command){
      this.state.dirty = true;
      var message = Array.isArray(command)?command:[command];
      console.log("Sending: ",JSON.stringify(message));
      axios.post('/commands',message)
      .catch(function (error) {
        console.log(error);
      })
    },
    resetSynth(){
      graph.clear();
      var command="Reset";
      this.sendCommand(command);
    }
  }

  return Store;
})
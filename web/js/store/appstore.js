define(function (require) {
  // Add global scopr graph
  require('litegraph/litegraph');

  let axios = require("axios");
  let graph = new LGraph();

  function findNodeWithId(nodeId){
    return graph._nodes.find(node => node.synth_infos.id == nodeId)
  }

  function findNodeWithInternalId(nodeId){
    console.log(graph._nodes)
    return graph._nodes.find(node => node.id == nodeId)
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
    savePatch(){
      console.log("Save");
      var commands = [];
      var metas = [];

      // Create
      graph._nodes.forEach(n => {
        commands.push({Create: { id: n.synth_infos.id, node_type: n.synth_infos.type }});
      });
      // Link   
      Object.keys(graph.links).forEach(linkId => {
        var { origin_id, origin_slot, target_id, target_slot} = graph.links[linkId];
        var node1 = findNodeWithInternalId(origin_id);
        var node2 = findNodeWithInternalId(target_id);
        commands.push({Link: {src_node: node1.synth_infos.id, src_port: origin_slot, dst_node: node2.synth_infos.id, dst_port: target_slot }});
      })
      // Config
      graph._nodes.forEach(n => {
        var type = n.synth_infos.type;
        var id = n.synth_infos.id;
        var typeSpec = this.findNodeType(type);
        typeSpec.config_spec.forEach(cs => {
          var {key, typ} = cs;
          var value;
          if (typ == "StringType") value = { StringVal: n.properties[key] }
          if (typ == "FloatType") value = { FloatVal: n.properties[key] }
          if (typ == "IntType") value = { IntVal: n.properties[key] }
          if (typ == "BoolType") value = { BoolVal: n.properties[key] }
          commands.push({ChangeConfig: { id: id,   key: key,   val:  value}})
        });
      });
      // Metas
      graph._nodes.forEach(n => {
        var id = n.synth_infos.id;
        var pos = n.pos;
        var title = n.title;
        metas.push({id,title,x: pos[0],y: pos[1]});
      })
      return axios
        .post('/patches/'+this.state.current_patch,{commands:commands,metas:metas})
        .then(c => {
          this.state.dirty = false;
          console.log("Saved!")
        })
        .then(c => {
          this.fetchPatches()
        })
    },
    selectPatch(patch){
      this.state.current_patch = patch
      this.resetSynth();
      if (patch == 'New') {
        this.state.dirty = false;
      } else {
        return axios.get('/patches/'+patch).then((response) => {
          var datas = response.data;
          var metas = datas.metas;
          var commands = datas.commands;
          commands
            .filter(c => c=="Reset")
            .forEach(c => this.applyResetCommand(c));
          commands
            .filter(c => c.Create)
            .forEach(c => this.applyCreateCommand(c,metas));
          commands
            .filter(c => c.ChangeConfig)
            .forEach(c => this.applyConfigCommand(c));
          commands
            .filter(c => c.Link)
            .sort((a,b) => a.Link.dst_node=="master"?1:-1)  // Link master at the end.
            .forEach(c => this.applyLinkCommand(c));
          this.state.dirty = false;
        })
      }
    },
    applyResetCommand(command){
        this.resetSynth();
    },
    applyCreateCommand(command,metas){
        var id = command.Create.id;
        var meta = metas.find(m => m.id == id);
        var pos = [100,100];         
        var title = null;
        if (meta){
          title = meta.title;
          pos = [meta.x,meta.y]; 
        }
        var nodeType = this.findNodeType(command.Create.node_type);
        if (nodeType == null){
          alert("Node type: "+nodeType+" not found.")
        }
        var node = LiteGraph.createNode(nodeType.classifier);
        node.synth_infos.id = id;
        node.pos = pos;
        if (title){
          node.title = title;
        }
        graph.add(node);
    },
    applyLinkCommand(command){
        var {src_node, src_port, dst_node, dst_port} = command.Link;
        var node1 = findNodeWithId(src_node);
        var node2 = findNodeWithId(dst_node);
        node1.connect(src_port, node2, dst_port );
    },
    applyConfigCommand(command){
        var { id, key, val } =  command.ChangeConfig;
        var value = null;
        if ("StringVal" in val) value = val.StringVal; 
        if ("FloatVal" in val) value = val.FloatVal;
        if ("IntVal" in val) value = val.IntVal;
        if ("BoolVal" in val) value = val.BoolVal;
        var node = findNodeWithId(id);
        node.onPropertyChanged(key,value);
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
      axios.post('/commands',["Reset"])
        .catch(function (error) {
          console.log(error);
        })
    },
    setDirty(val){
      this.state.dirty = val;
    }
  }

  return Store;
})
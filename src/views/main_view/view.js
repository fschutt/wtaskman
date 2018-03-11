var rpc = {
  invoke : function(arg) { window.external.invoke(JSON.stringify(arg)); },
  init : function() { rpc.invoke({cmd : 'init' }); },
  log : function() {
    var s = '';
    for (var i = 0; i < arguments.length; i++) {
      if (i != 0) {
        s = s + ' ';
      }
      s = s + JSON.stringify(arguments[i]);
    }
    rpc.invoke({cmd : 'log', text : s});
  },
  addTask : function(name) { rpc.invoke({cmd : 'addTask', name : name}); },
  clearDoneTasks : function() { rpc.invoke({cmd : 'clearDoneTasks'}); },
  markTask : function(index, done) {
    rpc.invoke({cmd : 'markTask', index : index, done : done});
  },
  render : function(items) {
    // return element = picodom.patch(oldNode, (oldNode = UI(items)), element);
  },
};

window.onload = function() { rpc.init(); };
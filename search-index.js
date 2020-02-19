var searchIndex={};
searchIndex["regular"] = {"doc":"Tools for manipulating regular languages.","i":[[3,"DFABuilder","regular","Builder for a DFA.",null,null],[3,"DefaultDFAStorage","","Default storage for a DFA.",null,null],[3,"DFA","","A deterministic finite automaton.",null,null],[4,"Error","","Errors from `regular` operations.",null,null],[13,"MissingStartState","","Start state was not specified.",0,null],[13,"InvalidState","","State specified was not valid for this automaton.",0,null],[13,"SymbolNotInAlphabet","","Symbol not found in alphabet.",0,null],[13,"StateNotFound","","State not found.",0,null],[13,"OperationWithNonEqualAlphabets","","Attempted to perform operation with two different alphabets.",0,null],[4,"Range","","Set of contiguous elements",null,null],[13,"NonEmpty","","Non-empty range",1,null],[12,"start","regular::Range","Start value, inclusive",1,null],[12,"end","","End value, inclusive",1,null],[13,"Empty","regular","Empty range",1,null],[0,"accept","","Generalization of the accept/non-accept of regular…",null,null],[8,"Accept","regular::accept","Trait for objects that can decide whether or not a string…",null,null],[16,"Symbol","","The type of symbols in the strings.",2,null],[10,"accept","","Return `true` if the given string is member of the…",2,[[["intoiterator"]],["bool"]]],[8,"IterExt","","Extension to the `Iterator` trait to test whether the…",null,null],[11,"is_accepted","","Return `true` if the string given by this iterator is a…",3,[[["accept"]],["bool"]]],[0,"alphabet","regular","Traits and implementations of generic sets of symbol,…",null,null],[3,"Boolean","regular::alphabet","Alphabet containing all booleans {True, False}.",null,null],[3,"Unit","","Alphabet containing single symbol of the unit type.",null,null],[3,"FullRange","","An object which can be turned into an alphabet which has…",null,null],[8,"Alphabet","","A set of symbols.",null,null],[16,"Symbol","","The type of elements in this set.",4,null],[16,"ValueIter","","An iterator over all values in this alphabet.",4,null],[10,"values","","Return an iterator over all value in this alphabet.",4,[[["self"]]]],[10,"contains","","Return `true` if the given symbol is a member of this…",4,[[["self"]],["bool"]]],[10,"num_values","","Optionally return the number of elements in this alphabet.…",4,[[["self"]],[["option",["usize"]],["usize"]]]],[8,"IntoAlphabet","","Coversion into an alphabet.",null,null],[16,"Symbol","","The type of symbols in the alphabet.",5,null],[16,"IntoAlpha","","The type of alphabet we are turning this into.",5,null],[10,"into_alphabet","","Create an alphabet from this value.",5,[[]]],[11,"intersection","regular","Construct a new DFA that accepts the regular language that…",6,[[["dfa"],["self"]],[["error"],["result",["dfa","error"]],["dfa"]]]],[11,"union","","Construct a new DFA that accepts the regular language that…",6,[[["dfa"],["self"]],[["error"],["result",["dfa","error"]],["dfa"]]]],[11,"difference","","Construct a new DFA that accepts the regular language that…",6,[[["dfa"],["self"]],[["error"],["result",["dfa","error"]],["dfa"]]]],[11,"complement","","Construct a new DFA that accepts the regular language that…",6,[[["self"]],["self"]]],[11,"accept_states","","The states of DFA that will cause it to accept a string.",6,[[["self"]]]],[11,"dead_state","","An optional state that signals early termination of the…",6,[[["self"]],["option"]]],[11,"start_state","","The starting state of the DFA.",6,[[["self"]]]],[11,"into_builder","","Convert this DFA back into the DFABuilder form.",6,[[],["dfabuilder"]]],[11,"accept","","Accept or reject a string based on the content of this DFA.",6,[[["intoiterator"],["self"]],["bool"]]],[11,"accept_unchecked","","Accept or reject a string based on the content of this…",6,[[["intoiterator"],["self"]],["bool"]]],[11,"new","","Construct a new default storage with the given alphabet.",7,[[["a"]],["self"]]],[11,"new","","Create a new DFABuilder with the given alphabet.",8,[[["intoalphabet"]],["self"]]],[11,"new_with_storage","","Create a new DFABuilder with a customer storage backend.",8,[[["s"]],["self"]]],[11,"storage","","Return a reference to the `DFAStorage` backing this builder.",8,[[["self"]],["s"]]],[11,"alphabet","","Return a reference to the `Alphabet` in the storage of…",8,[[["self"]],["a"]]],[11,"new_state","","Record and return a new state.",8,[[["self"]]]],[11,"transition","","Record and validate a new transition.",8,[[["self"]],[["result",["error"]],["error"]]]],[11,"transitions","","Record and validate multiple transitions.",8,[[["self"]],[["result",["error"]],["error"]]]],[11,"accept_states","","Add to the set of accept states.",8,[[["self"]],["self"]]],[11,"start_state","","Set the starting state.",8,[[["self"]],["self"]]],[11,"dead_state","","Set the dead/error state.",8,[[["option"],["self"]],["self"]]],[11,"build","","Build the",8,[[],[["error"],["result",["dfa","error"]],["dfa"]]]],[11,"contains","","Returns `true` if the given elements is within the range.",1,[[["sym"],["self"]],["bool"]]],[0,"prelude","","Common items to import.",null,null],[3,"DFABuilder","regular::prelude","Builder for a DFA.",null,null],[3,"DefaultDFAStorage","","Default storage for a DFA.",null,null],[3,"DFA","","A deterministic finite automaton.",null,null],[4,"Error","","Errors from `regular` operations.",null,null],[13,"MissingStartState","","Start state was not specified.",0,null],[13,"InvalidState","","State specified was not valid for this automaton.",0,null],[13,"SymbolNotInAlphabet","","Symbol not found in alphabet.",0,null],[13,"StateNotFound","","State not found.",0,null],[13,"OperationWithNonEqualAlphabets","","Attempted to perform operation with two different alphabets.",0,null],[8,"DFAStorage","","Backend for the DFA struct.",null,null],[16,"State","","Type representing a state of the DFA.",9,null],[10,"from_alphabet","","Construct a new instance of this storage from the provided…",9,[[["a"]],["self"]]],[10,"alphabet","","Return a reference to the alphabet used by this DFA.",9,[[["self"]],["a"]]],[10,"all_states","","Return a list of all the valid states of this DFA.",9,[[["self"]],["vec"]]],[10,"all_transitions","","Return a list of all transitions of this DFA.",9,[[["self"]],["vec"]]],[10,"contains_state","","Return `true` if the given state is valid in this DFA.",9,[[["self"]],["bool"]]],[10,"transition","","Return `Some(end)` if there exists a transition from…",9,[[["self"]],["option"]]],[11,"transition_unchecked","","Return the resulting state from transitioning from the…",9,[[["self"]]]],[10,"add_state","","Return a new unique state.",9,[[["self"]]]],[10,"add_transition","","Record the given transition.",9,[[["self"]]]],[8,"DFAStorage","regular","Backend for the DFA struct.",null,null],[16,"State","","Type representing a state of the DFA.",9,null],[10,"from_alphabet","","Construct a new instance of this storage from the provided…",9,[[["a"]],["self"]]],[10,"alphabet","","Return a reference to the alphabet used by this DFA.",9,[[["self"]],["a"]]],[10,"all_states","","Return a list of all the valid states of this DFA.",9,[[["self"]],["vec"]]],[10,"all_transitions","","Return a list of all transitions of this DFA.",9,[[["self"]],["vec"]]],[10,"contains_state","","Return `true` if the given state is valid in this DFA.",9,[[["self"]],["bool"]]],[10,"transition","","Return `Some(end)` if there exists a transition from…",9,[[["self"]],["option"]]],[11,"transition_unchecked","regular::prelude","Return the resulting state from transitioning from the…",9,[[["self"]]]],[10,"add_state","regular","Return a new unique state.",9,[[["self"]]]],[10,"add_transition","","Record the given transition.",9,[[["self"]]]],[11,"from","","",8,[[["t"]],["t"]]],[11,"into","","",8,[[],["u"]]],[11,"to_owned","","",8,[[["self"]],["t"]]],[11,"clone_into","","",8,[[["self"],["t"]]]],[11,"try_from","","",8,[[["u"]],["result"]]],[11,"try_into","","",8,[[],["result"]]],[11,"borrow","","",8,[[["self"]],["t"]]],[11,"borrow_mut","","",8,[[["self"]],["t"]]],[11,"type_id","","",8,[[["self"]],["typeid"]]],[11,"from","","",7,[[["t"]],["t"]]],[11,"into","","",7,[[],["u"]]],[11,"to_owned","","",7,[[["self"]],["t"]]],[11,"clone_into","","",7,[[["self"],["t"]]]],[11,"try_from","","",7,[[["u"]],["result"]]],[11,"try_into","","",7,[[],["result"]]],[11,"borrow","","",7,[[["self"]],["t"]]],[11,"borrow_mut","","",7,[[["self"]],["t"]]],[11,"type_id","","",7,[[["self"]],["typeid"]]],[11,"from","","",6,[[["t"]],["t"]]],[11,"into","","",6,[[],["u"]]],[11,"to_owned","","",6,[[["self"]],["t"]]],[11,"clone_into","","",6,[[["self"],["t"]]]],[11,"try_from","","",6,[[["u"]],["result"]]],[11,"try_into","","",6,[[],["result"]]],[11,"borrow","","",6,[[["self"]],["t"]]],[11,"borrow_mut","","",6,[[["self"]],["t"]]],[11,"type_id","","",6,[[["self"]],["typeid"]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[],["u"]]],[11,"to_owned","","",0,[[["self"]],["t"]]],[11,"clone_into","","",0,[[["self"],["t"]]]],[11,"to_string","","",0,[[["self"]],["string"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"try_into","","",0,[[],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"type_id","","",0,[[["self"]],["typeid"]]],[11,"into_alphabet","","",1,[[]]],[11,"from","","",1,[[["t"]],["t"]]],[11,"into","","",1,[[],["u"]]],[11,"into_iter","","",1,[[],["i"]]],[11,"to_owned","","",1,[[["self"]],["t"]]],[11,"clone_into","","",1,[[["self"],["t"]]]],[11,"try_from","","",1,[[["u"]],["result"]]],[11,"try_into","","",1,[[],["result"]]],[11,"borrow","","",1,[[["self"]],["t"]]],[11,"borrow_mut","","",1,[[["self"]],["t"]]],[11,"type_id","","",1,[[["self"]],["typeid"]]],[11,"into_alphabet","regular::alphabet","",10,[[]]],[11,"from","","",10,[[["t"]],["t"]]],[11,"into","","",10,[[],["u"]]],[11,"to_owned","","",10,[[["self"]],["t"]]],[11,"clone_into","","",10,[[["self"],["t"]]]],[11,"try_from","","",10,[[["u"]],["result"]]],[11,"try_into","","",10,[[],["result"]]],[11,"borrow","","",10,[[["self"]],["t"]]],[11,"borrow_mut","","",10,[[["self"]],["t"]]],[11,"type_id","","",10,[[["self"]],["typeid"]]],[11,"into_alphabet","","",11,[[]]],[11,"from","","",11,[[["t"]],["t"]]],[11,"into","","",11,[[],["u"]]],[11,"to_owned","","",11,[[["self"]],["t"]]],[11,"clone_into","","",11,[[["self"],["t"]]]],[11,"try_from","","",11,[[["u"]],["result"]]],[11,"try_into","","",11,[[],["result"]]],[11,"borrow","","",11,[[["self"]],["t"]]],[11,"borrow_mut","","",11,[[["self"]],["t"]]],[11,"type_id","","",11,[[["self"]],["typeid"]]],[11,"into_alphabet","","",12,[[]]],[11,"from","","",12,[[["t"]],["t"]]],[11,"into","","",12,[[],["u"]]],[11,"try_from","","",12,[[["u"]],["result"]]],[11,"try_into","","",12,[[],["result"]]],[11,"borrow","","",12,[[["self"]],["t"]]],[11,"borrow_mut","","",12,[[["self"]],["t"]]],[11,"type_id","","",12,[[["self"]],["typeid"]]],[11,"values","regular","",1,[[["self"]]]],[11,"contains","","",1,[[["self"]],["bool"]]],[11,"num_values","","",1,[[["self"]],[["option",["usize"]],["usize"]]]],[11,"values","regular::alphabet","",10,[[["self"]]]],[11,"contains","","",10,[[["self"]],["bool"]]],[11,"num_values","","",10,[[["self"]],[["option",["usize"]],["usize"]]]],[11,"values","","",11,[[["self"]]]],[11,"contains","","",11,[[["self"]],["bool"]]],[11,"num_values","","",11,[[["self"]],[["option",["usize"]],["usize"]]]],[11,"into_alphabet","","",12,[[]]],[11,"from_alphabet","regular","",7,[[["a"]],["self"]]],[11,"all_states","","",7,[[["self"]],["vec"]]],[11,"all_transitions","","",7,[[["self"]],["vec"]]],[11,"transition","","",7,[[["self"]],["option"]]],[11,"add_state","","",7,[[["self"]]]],[11,"add_transition","","",7,[[["self"]]]],[11,"contains_state","","",7,[[["self"]],["bool"]]],[11,"alphabet","","",7,[[["self"]],["a"]]],[11,"next","","",1,[[["self"]],["option"]]],[11,"clone","regular::alphabet","",10,[[["self"]],["boolean"]]],[11,"clone","","",11,[[["self"]],["unit"]]],[11,"clone","regular","",6,[[["self"]],["dfa"]]],[11,"clone","","",7,[[["self"]],["defaultdfastorage"]]],[11,"clone","","",8,[[["self"]],["dfabuilder"]]],[11,"clone","","",0,[[["self"]],["error"]]],[11,"clone","","",1,[[["self"]],["range"]]],[11,"eq","regular::alphabet","",10,[[["boolean"],["self"]],["bool"]]],[11,"eq","","",11,[[["self"],["unit"]],["bool"]]],[11,"eq","regular","",1,[[["self"],["range"]],["bool"]]],[11,"ne","","",1,[[["self"],["range"]],["bool"]]],[11,"fmt","regular::alphabet","",10,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",11,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",12,[[["self"],["formatter"]],["result"]]],[11,"fmt","regular","",6,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",7,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",8,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",0,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",1,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",0,[[["formatter"],["self"]],["result"]]],[11,"hash","regular::alphabet","",10,[[["self"],["__h"]]]],[11,"hash","","",11,[[["self"],["__h"]]]],[11,"hash","regular","",1,[[["self"],["__h"]]]],[11,"transition_unchecked","regular::prelude","Return the resulting state from transitioning from the…",9,[[["self"]]]]],"p":[[4,"Error"],[4,"Range"],[8,"Accept"],[8,"IterExt"],[8,"Alphabet"],[8,"IntoAlphabet"],[3,"DFA"],[3,"DefaultDFAStorage"],[3,"DFABuilder"],[8,"DFAStorage"],[3,"Boolean"],[3,"Unit"],[3,"FullRange"]]};
addSearchOptions(searchIndex);initSearch(searchIndex);
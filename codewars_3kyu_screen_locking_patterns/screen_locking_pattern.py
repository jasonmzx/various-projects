def count_patterns_from(firstPoint, length):
    # length 0 or 10 and bigger...
    print(f"fP: {firstPoint} , len: {length}")
    if length >= 10 or length == 0:
        return 0
    if length == 1:
        return 1
    # Movement possibilities;
    mov_possib = {
        "A": ["B", "D", "F", "H", "E"],
        "B": ["A", "C", "E", "D", "F", "I", "G"],
        "C": ["B", "F", "E", "D", "H"],
        "D": ['A', 'G', 'E', 'B', 'H', 'C', 'I'],
        "E": ['A', 'B', 'C', 'D', 'F', 'G', 'H', 'I'],
        "F": ['C', 'I', 'E', 'B', 'H', 'A', 'G'],
        "G": ['D', 'H', 'E', 'F', 'B'],
        "H": ["G", 'D', 'E', 'F', 'I', 'C', 'A'],
        "I": ['H', 'E', 'F', 'B', 'D']
    }
    # Passing thru elements possibilities;
    pass_thru = {
        "B": [['A', 'C']],
        "F": [['C', 'I']],
        "H": [['G', 'I']],
        "D": [['A', 'G']],
        "E": [['B', 'H'], ['D', 'F'], ['C', 'G'], ['A', 'I']]
    }

    # Return possibilites;
    ret_num = 0
    # Previous Nodes used:
    stages = []
    for i in range(length-1):
        temp_stage = []
        if i == 0:
            temp_stage.append([ [ [], firstPoint, mov_possib[firstPoint] ] ] )
            stages.append(temp_stage)
            pass
        else:
            for subsets in stages[i-1]:
                for array in subsets:
                    #Used nodes for the array currently being generated..
                    prev_node = array[0][:]
                    prev_node.append(array[1])
                    temp_set = []
                    for j in array[2]:
                        temp_move = []
                        for k in mov_possib[j]:
                            if k in prev_node:
                                if k in pass_thru.keys():
                                    for ptn in pass_thru[k]:
                                        if j in ptn:
                                            for z in ptn:
                                                if z is not j and z not in prev_node:
                                                    temp_move.append(z)
                            else:
                                temp_move.append(k)

                        temp_set.append( [ prev_node,j,temp_move ] )
                    temp_stage.append(temp_set)
            stages.append(temp_stage)


    for test in stages[-1]:
        for x in test:
            ret_num += len(x[2])
    return ret_num
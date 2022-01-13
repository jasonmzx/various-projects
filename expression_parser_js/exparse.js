

//Bracket: 0
//DM_O : 1
//AS_O : 2



//PEDMAS
const tokenRef = {
    bracket : ['(' , ')'],
    dm_o : ['/' , '*'], //Division / Multiplication Operators (P) (priority)
    as_o : ['+' , '-'], //Addition / Substraction Operators (S) (secondary)
}

const NumParse = (ei,i,expression) => {

    if(i+1 === expression.length){
        return i+1
    }

    for(let j = i+1 ; j < expression.length; j++){
        
        if(isNaN(expression[j]) && expression[j] !== '.'){
            return j;
        }


    }
} //endof NumParse



const EndBracket = (tokenObj) => {

    let depthCount = 1;

    for(let i = 0 ; i < tokenObj.length; i++){
        const obj = tokenObj[i];


        if(obj.string === '('){
            depthCount += 1;
        }

        if(obj.string === ')'){
            depthCount -= 1;
        }

        if(depthCount === 0){
            return i
        }

    }


}

const Operation = (a,b,o) => {
    switch(o){
        case '*':
            return parseFloat(a) * parseFloat(b);

        case '/':
            return parseFloat(a) / parseFloat(b)

        case '+':
            return parseFloat(a) + parseFloat(b)

        case '-':
            return parseFloat(a) - parseFloat(b)

    }
}



var calc = function (expression) {
    expression = expression.replace(/\s/g, ""); //Whitespace removal using re
    console.log(expression);

    //tokenize

    let Tokenized = [];

    for(let i = 0; i < expression.length; i++){
        
        const eI = expression[i];

        switch(isNaN(eI)){
            case true: //Isn't num

                if(tokenRef.bracket.includes(eI) ) { //If Brackets
                    Tokenized.push({
                        // start: i, 
                        // end: i+1, 
                        type: 0, 
                        string: expression.slice(i,i+1)
                    }
);
                    continue;
                } 

                if(tokenRef.as_o.includes(eI)) { //Secondary
                    if(eI === '-' && !isNaN(expression[i+1]) ){
                        continue;
                    }

                    Tokenized.push({
                        // start: i, 
                        // end: i+1, 
                        type: 2, 
                        string: expression.slice(i,i+1)
                    });
                    continue;
                }

                if(tokenRef.dm_o.includes(eI)) { //Primary

                    Tokenized.push({
                        // start: i, 
                        // end: i+1, 
                        type: 1, 
                        string: expression.slice(i,i+1)
                    });


                }


                break;
            case false: //Is Number

                if( ( isNaN(expression[i-1]) && expression[i-1] !== '.') || i === 0){

                    //(start,finish,type)
                    Tokenized.push({
                        type: -1,
                        string: expression.slice(
                            expression[i-1] === '-' ? i-1 : i,
                            NumParse(eI,i,expression
                                ))

                    });

                }

                break;
            default:
                break;
        }


    } //end of Tokenizing

    const expressionEval = (tokenObj,depth) => {
        console.log('Obj : '+depth);
        console.log(tokenObj);

        //Bracket Parsing
        
        let brackCount = 0;

        let resultStore = [];

        for (const [i, v] of tokenObj.entries()) {
            if(v.string === '('){
                brackCount += 1;
                const EB = EndBracket(tokenObj.slice(i+1,tokenObj.length));
                
                if(brackCount === depth || brackCount === depth+1){
                    let R = expressionEval(tokenObj.slice(i+1,EB+i+1),depth+1);
                    
                    resultStore.push({
                        s: i+1, //start
                        e: EB+1, //end
                        r : R[0] //result
                    })

                    console.log('Incoming R:')
                    console.log(R);
                    console.log('Deets: '+(i+1)+' '+(EB+i+1));
                }


            } else if(v.string === ')'){
                brackCount -= 1;
            }
        } //end of bracket parse

        console.log(resultStore);  

        //bracket refactor
        for(const [r,v] of resultStore.entries()){
            
            tokenObj.splice(v.s,v.e);
            tokenObj[v.s-1] = v.r;

        }

        console.log(tokenObj);


        // for(let i = 0; i < [...tokenObj].length; i++) {
        //     const v = tokenObj[i];
        //     if(v.string === '('){
        //         brackCount += 1;
        //         const EB = EndBracket(tokenObj.slice(i+1,tokenObj.length));
                
        //         if(brackCount === depth || brackCount === depth+1){
        //             let R = expressionEval(tokenObj.slice(i+1,EB+i+1),depth+1);
                    
        //              tokenObj.splice(i+1,EB+i+1);
        //              tokenObj[i] = R[0];
        //             i = i-1
        //             console.log('Incoming R:')
        //             console.log(R);
        //             console.log(tokenObj);
        //             console.log('Deets: '+(i+1)+' '+(EB+i+1));
        //         }


        //     } else if(v.string === ')'){
        //         brackCount -= 1;
        //     }
        // }

        //Primary Operations
        for(let p = 0; p < [...tokenObj].length; p++){
            const obj = tokenObj[p];

            if(tokenRef.dm_o.includes(obj.string)){ //If * or /

                console.log(tokenObj[p-1].string);
                console.log(obj.string);
                console.log(tokenObj[p+1].string);
            
                const result = Operation(
                    tokenObj[p-1].string, //a
                    tokenObj[p+1].string, //b
                    obj.string); //operator

                console.log(result);
                    
                tokenObj.splice(p,2);
                tokenObj[p-1] = { type: -1, string: result.toString()}
                console.log(tokenObj)

                p = p-1
                
            }
        }


        //Secondary Operations
        for(let p = 0; p < [...tokenObj].length; p++){
            const obj = tokenObj[p];

            if(tokenRef.as_o.includes(obj.string)){ //If * or /
            
                const result = Operation(
                    tokenObj[p-1].string, //a
                    tokenObj[p+1].string, //b
                    obj.string); //operator

                tokenObj.splice(p,2);
                tokenObj[p-1] = { type: -1, string: result.toString()}
                //console.log(tokenObj)

                p = p-1
                
            }
        }


        return tokenObj
    }

    console.log(expressionEval(Tokenized,0));

    //console.log(Tokenized);


    return -12
};


//calc('1+( (1+1) - (0.5 + 0.5))');

calc('5*(2+ -4/(1.1+-2) - (2+3) + 5 )');

//calc('5*(2+-4/(1+1))+2.5')
//calc('5/( (1+1) + (2- -2*3.1*2) )+4.344-(-2.2+1)')

// 12*123/-(-5+2.5)
// 01234567890123456
// (2/(2+3.33)*4)--6

//How it should be solved:

// (2/(2+3.33)*4)--6

// ( 2 / (5.33) *4 ) --6
// ( 2 / 5.33 * 4 )
// 0.37523452157 * 4 + 6
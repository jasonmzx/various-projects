

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

const reducerSum = (pV, cV) => pV + cV;

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
                    if(eI === '-' && !isNaN(expression[i+1]) ){ //if - is part of negative float
                        continue;
                    } 

                    else if(eI === '-' && expression[i+1] === '('){
                        Tokenized.push({ //distributive modifier token (type 3)
                            type: 3,
                            string: '-m'
                        })
                        continue
                    } else {

                    Tokenized.push({
                        // start: i, 
                        // end: i+1, 
                        type: 2, 
                        string: expression.slice(i,i+1)
                    });
                    continue;
                    }
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

   // console.log('TOKENZIED !')
 //   console.log(Tokenized);
    //console.log('//TOKENZIED !')



    const expressionEval = (tokenObj,depth) => {
      //  console.log('Obj : '+depth);
      //  console.log(tokenObj);

        //Bracket Parsing
        
        let brackCount = depth;

        let resultStore = [];

        for (const [i, v] of tokenObj.entries()) {
            if(v.string === '('){
                brackCount += 1;
                const EB = EndBracket(tokenObj.slice(i+1,tokenObj.length));
                
               // console.log('brackCount: '+brackCount+' depth: '+depth + ' index: '+i);
                if(brackCount === depth || brackCount === depth+1){
                    let R = expressionEval(tokenObj.slice(i+1,EB+i+1),depth+1);
                    
                    resultStore.push({
                        start: i+1, //start
                        end: EB+1, //end (relative to start)
                        result : R[0] //result
                    })

                  //  console.log('Incoming R:')
                   // console.log(R);
                  //  console.log('Deets: '+(i+1)+' '+(EB+i+1));
                }


            } else if(v.string === ')'){
                brackCount -= 1;
            }
        } //end of bracket parse

      //  console.log('Result STORE !');
      //  console.log(resultStore);  
       // console.log('\n');

        //bracket refactor
        for(const [r,v] of resultStore.entries()){

            const offset = resultStore.slice(0, r).map((e) => {return e.end}).reduce(reducerSum,0);
         //   console.log('REFAC');
            tokenObj.splice((v.start-offset),v.end);
            tokenObj[v.start-offset-1] = v.result;

        }
        
        //console.log('After refactor for depth: '+depth);
        //console.log(tokenObj);


        //Distributive modifiers applications
        //console.log('Looking at:')
        //console.log(tokenObj)

        for(let d = 0; d < [...tokenObj].length; d++){
            const obj = tokenObj[d];

            if(obj.type === 3 && tokenObj[d+1].type === -1){
               // console.log('Passed thru');

                const modifiedValue = parseFloat(tokenObj[d+1].string)*-1;
               // console.log(modifiedValue);

                if(d != 0 && tokenObj[d-1].type === -1){
                    tokenObj[d] = {type: 2, string: '+'};
                    tokenObj[d+1] = {type: -1, string: (modifiedValue).toString() };
                } else {
                   // console.log('Elsed out');
                    tokenObj[d+1] = {type: -1, string: (modifiedValue).toString() };
                    tokenObj.splice(d,1);
                }
                d = d-1;


            }
            

        }

        //console.log('passthru after:')
        //console.log(tokenObj);

        //Primary Operations
        for(let p = 0; p < [...tokenObj].length; p++){
            const obj = tokenObj[p];

            if(obj.type == 1){ //If * or /

               // console.log(tokenObj[p-1].string);
               // console.log(obj.string);
               // console.log(tokenObj[p+1].string);
            
                const result = Operation(
                    tokenObj[p-1].string, //a
                    tokenObj[p+1].string, //b
                    obj.string); //operator

               // console.log(result);
                    
                tokenObj.splice(p,2);
                tokenObj[p-1] = { type: -1, string: result.toString()}
              //  console.log(tokenObj)

                p = p-1
                
            }
        }


        //Secondary Operations
        for(let p = 0; p < [...tokenObj].length; p++){
            const obj = tokenObj[p];

            if(obj.type === 2 || obj.string[0] === '-' && p !== 0 && tokenObj[p-1].type === -1){ //If + ot -
                
                let result = null;

                if(obj.type == -1){ //If 
                    result = Operation(
                        tokenObj[p-1].string,
                        obj.string,
                        '+'
                    );

                    tokenObj.splice(p,1);
                } else {
                    result = Operation(
                        tokenObj[p-1].string, //a
                        tokenObj[p+1].string, //b
                        obj.string); //operator
    
                    tokenObj.splice(p,2);
                }


                tokenObj[p-1] = { type: -1, string: result.toString()}
                //console.log(tokenObj)

                p = p-1
                
            }
        }

        console.log(tokenObj);
        return tokenObj
    }




    return parseInt(expressionEval(Tokenized,0)[0].string);

};


//calc('1+( (1+1) - (0.5 + 0.5))');

//calc('(2+2) + 23/(2*(21+2.2))');


//calc('2+(10*10)')
//calc('2-3');
//calc('2 / (2 + 3) * 4.33 - -6');
//calc('(1 - 2) + -(-(-(-4)))'); //Flawed

//calc('(1 - 2) + -1*(-1*(-1*(-4)))'); //Flawed
console.log('Val:')
console.log(calc('12* 123/-(-5 + 2)')); //flawed

//calc('5*(2+-4/(1+1))+2.5')
//calc('-(-1+2)');
//calc('5/( (1+1) + (2- -2*3.1*2) )+4.344-(-2.2+1)')

// 12*123/-(-5+2.5)
// 01234567890123456
// (2/(2+3.33)*4)--6

//How it should be solved:

// (2/(2+3.33)*4)--6

// ( 2 / (5.33) *4 ) --6
// ( 2 / 5.33 * 4 )
// 0.37523452157 * 4 + 6
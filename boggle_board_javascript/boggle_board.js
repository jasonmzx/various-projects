function checkWord(board, word) {
  //Edge cases:
  if(word.length == 1){
    if(board.some(elm => elm.includes(word))){
      return true
    } else{ return false}
  }
  
  //Initial start sequence:
    let firstLetter = []
    board.map((row,rowIndex) => {
      row.map( (col,colIndex) => {  
        if(word[0] === col){
          firstLetter.push([rowIndex, colIndex]);
        }
      });
    });
    
    //Recursive function:
    let surroundCheck = (coord, board, word, passedCoords) => {

      const neighbours = [ [-1,-1], [-1,0], [-1,1], [0,-1], [0,1], [1,-1], [1,0], [1,1] ]  
      
      let validNext = []
      for(const offset of neighbours){
              
      if(coord[0]+ offset[0] < 0 || coord[0]+offset[0] > board.length-1 || //Y
        coord[1]+offset[1] < 0 || coord[1]+offset[1] > board[0].length-1){} //X
      else {
        if(   board[coord[0]+offset[0]][coord[1]+offset[1]] == word[0] 
           && !(passedCoords.some(elm => elm[0] == coord[0]+offset[0] && elm[1] == coord[1]+offset[1])) 
          ){
          validNext.push( [coord[0]+offset[0] , coord[1]+offset[1]] );
        }
      }
        
      };
      
     if(word.length === 1 && validNext.length > 0){
        return true
       }  
      
    
    for(const validCoord of validNext){
      passedCoords.push(validCoord);
      if(surroundCheck(validCoord, board, word.slice(1), passedCoords) === true){
        return true
      };
    };      
      
    } 
    //Recursive callback (In the IF statement)
    for(const firstCoord of firstLetter){
      const passedCoord = [firstCoord]
      
      if(surroundCheck(firstCoord,board, word.slice(1),passedCoord) === true){
        return true
      }
    };  
  return false
} 

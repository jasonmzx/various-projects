package objects;

import java.util.ArrayList;

public class KarnaughMap {

	//Class Properties:
	
	//Actual K-Map , 2D Array of boolean values:
	ArrayList<ArrayList<Boolean>> kmap = new ArrayList();
	boolean isKmapSquare;
	
	//Kmap Horiz & Verti headings ( 00, 01, 10, 11)
	ArrayList<Byte> horizontal = new ArrayList(); 
	ArrayList<Byte> vertical   = new ArrayList(); 
	
	//TODO: Implement an algorithm to generate this series (And make sure that only 1 bit is changed per entry)
		//Ex. 00 01 11 10 Is OK, 
		// 00 01 -> 10 11 IS NOT 01, 10 2 bits changed!!
	
	//Hardcodes for headings (Based on N#. Variables) 
	//Supports up to 3 Variate
	public static ArrayList<Byte> getHeading(int size) {
		
		ArrayList<Byte> ret = new ArrayList(); 
		
		if(size == 1) {	 //2^1 = 2 entries
			ret.add(new Byte((byte)0)); //0
			ret.add(new Byte((byte)1)); //1
			
		} else if (size == 2) { //2^2 = 4 entries
			ret.add(new Byte((byte)0)); //00
			ret.add(new Byte((byte)1)); //01
			ret.add(new Byte((byte)3)); //11	
			ret.add(new Byte((byte)2)); //10
		} else if (size == 3) { //2^3 = 8 entries
			
			ret.add(new Byte((byte)0)); //000
			ret.add(new Byte((byte)1)); //001
			ret.add(new Byte((byte)3)); //011
			ret.add(new Byte((byte)2)); //010
			ret.add(new Byte((byte)4)); //100
			ret.add(new Byte((byte)5)); //101
			ret.add(new Byte((byte)7)); //111		
			ret.add(new Byte((byte)6)); //110
		}
		
		return ret;		
	}
	
	
	//CONSTRUCTOR
	public KarnaughMap(ArrayList<ArrayList<Boolean>> km, boolean isSquare) {
		this.kmap = km;
		this.isKmapSquare = isSquare;
	}
	
}

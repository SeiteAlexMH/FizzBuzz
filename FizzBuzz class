import java.util.ArrayList;

public class FizzBuzz {
    ArrayList<Integer> divider = new ArrayList<Integer>();
	ArrayList<String> outputs = new ArrayList<String>();
	
	
	public void count(){
		for(int i = 1; i<101; i++){
			String display = "";
			for(int x=0; x<divider.size(); x++){
				if( i%divider.get(x)==0){
					display += outputs.get(x);
				}
			}
			if (display == ""){
				display += i;
			}
			System.out.println(display);
		}
		
	}
	
	public static void main(String[] args){
		FizzBuzz test = new FizzBuzz();
		test.divider.add(3);
		test.divider.add(5);
		test.outputs.add("Fizz");
		test.outputs.add("Buzz");
		test.count();
	}
}

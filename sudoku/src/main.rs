use std::thread;
use std::collections::HashMap;
use std::collections::HashSet;
use std:iter::Iterator;

//Macros
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

macro_rules! hashset {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashSet::new();
         $( map.insert($key, $val); )*
         map
    }}
}

//General solver error
enum SolveError { InvalidCell }


//General solver interface
trait Solver {
	fn Solved(&self) -> Result<bool,SolveError>;
	fn reducePossible(&self) -> Result<bool,SolveError>;
	fn validate(&self) -> bool
}

fn BoolHashSetFromIterator(it: Iterator) HashSet{
	return hashset!it.map( |x| x=>true).collect::<Vec<_>>();
}

fn BoolHashMapFromIterator(it: Iterator) HashSet{
	return hashmap!it.map( |x| x=>true).collect::<Vec<_>>();
}

fn validate(known: &[u8]) -> bool{
	
	let mut validNums = BoolHashMapFromIterator((1..9));
	
	for k in known{
		if validNums[k]{
			validNums[v] = false
		}else{
			return false 
		}
	}
	
	return true
}

//Cells which store individual numbers in the Grid
struct cell {
	m_possible 	: HashSet<u8,bool>, 
	m_x,m_y 	: u8
}

impl cell {

	fn new(x,y : u8) -> cell{
		cell {
			m_possible:BoolHashSetFromIterator((1..9)), 
			m_x:x,
			m_y:y
		}
	}
	
	fn validate(&self) -> bool{
		if self.m_possible.len() < 1{
			return false
		}
		if self.m_possible.len() > 9{
			return false
		}
		
		true
	}
	
	fn SetKnownTo(&self, value : u8){
		self.m_possible = hashset![value => true];
	}
	
	fn TakeKnownFromPossible(&self, known: &[u8]) -> Result<bool,SolveError>{
	
		var possibles = self.m_possible.len();
		
		if !self.IsKnown(){		
			self.m_possible = self.m_possible.intersection(BoolHashSetFromIterator(known)).collect::<HashSet<u8,bool>>();
			
			if !self.validate(){
				//errorStr := fmt.Sprintf("TakeKnownFromPossible() made cell invalid: Known:%T:%q", known,known)
				return Err(SolveError::InvalidCell);
			}
		}
		
		Ok(possibles != self.m_possible.len()) //did we take any?
	}
	
	fn IsKnown(&self) -> bool{
		self.m_possible.len() == 1
	}
	
	fn Known(&self) -> Result<u8, SolveError> { //Up to here
		
		// by convention we delete from the map possibles that are no longer possible
		// so we just need to check map length to see if the cell is solved
		if len(c.m_possible)!=1{
			return 0,errors.Wrap(SolveError{"Value not yet known for this cell"},1)
		}
		
		//Only one key is now considered "possible", it's value should be true, and it should
		//be the only one in the list, return it if that is the case 
		for k,v := range c.m_possible {
			if v{
				return k,nil
			}
		}
		
		return 0,errors.Wrap(SolveError{"Error in cell storage of known values"},1)
	}
	
	func (c *cell) Possibles() ([]int,error){
		possibles := make([]int,0,len(c.m_possible))
		
		for k,v := range c.m_possible{
			if v{
				possibles = append(possibles,k)
			}
		}
		
		return possibles,nil
	}
	
	func (c cell) String() string {
		val,err := c.Known()
		if(err != nil){
			return "x"
		}
		
		return strconv.Itoa(val)
	}
}

type cellPtrSlice []*cell 

func (cells cellPtrSlice) Solved() (bool, error){
	for _,c := range cells{
		if !c.IsKnown(){
			return false,nil
		}
	}
	
	return true,nil
}

func (cells cellPtrSlice) Known() ([]int, error){
	known := make([]int,0, len(cells))
	for _,c := range cells{
		if c.IsKnown(){
			val,err := c.Known()
			if err != nil{
				return known,err
			}
			known = append(known,val)
		}
	}
	
	return known,nil
}

func (cells cellPtrSlice) TakeKnownFromPossible(known []int) (bool,error){
	
	changed := false
	for _,c := range cells{
		taken, err := c.TakeKnownFromPossible(known)
		if err != nil{
			return false,err
		}
		changed = changed || taken
	}
	
	return changed,nil
}

//Squares which represent one of each of the 9 squares in a Grid, each of which 
//references a 3x3 collection of cells.

type square struct {
	m_cells [][]*cell
}

func (s square) Solved() (bool,error) {
	for _,r := range s.m_cells{
		solved,err := cellPtrSlice(r).Solved()
		if(err != nil){
			return false,err
		}
		if !solved {
			return false,nil
		}
		
	}
	return true,nil
}
func (s *square) init() {
	s.m_cells = make([][]*cell,SQUARE_SIZE)
	for i,_ := range s.m_cells{
		s.m_cells[i] = make(cellPtrSlice, SQUARE_SIZE)
	}
}

func (s* square) KnownInSquare() ([]int,error){
	known := make([]int,0,SQUARE_SIZE*SQUARE_SIZE)
	for x,_ := range s.m_cells{
		for y,_ := range s.m_cells[x]{
			c := s.m_cells[x][y]
			if c.IsKnown(){
				val,err := c.Known()
				if err != nil{
					return known,err
				}
				known = append(known,val)	
			}	
		}
	}
	
	return known,nil
}

func (s* square) reducePossible() (bool,error) {
	known,err := s.KnownInSquare()
	reduced := false
	if err != nil {
		return false,err
	}
	
	for x,_ := range s.m_cells{
		cells := s.m_cells[x]
		changed, err := cellPtrSlice(cells).TakeKnownFromPossible(known)
		if err != nil{
			return false,err
		}
		reduced = reduced || changed
	}
	return reduced,nil
}

func (s* square) validate() bool{
	known,err := s.KnownInSquare()
	if err != nil {
		return false
	}
	
	return validate(known)
}

type pair struct{
	m_values 		[2]int
	m_rowAligned	bool
	m_rowOrColNum	int
}

func (s* square) AlignedCellOnlyPairs() ([]pair,error){
	
	foundPairs := make([]pair,0,SQUARE_SIZE*SQUARE_SIZE)
	/*
	for x1,_ := range s.m_cells{
		for y1,_ := range s.m_cells[x1]{
			c1 := s.m_cells[x1][y1]
			if len(c1.m_possible)==2{
				for x2,_ := range s.m_cells{
					for y2,_ := range s.m_cells[x2]{
						c2 := s.m_cells[x2][y2]
						if c1 != c2{
							if len(c2.m_possible)==2{
								if x2 == x1 || y2 == y1{
									matchBoth := true
									valIdx :=0
									var foundPair pair
									for k,v := range c2.m_possible{
										if val,ok :=c1.m_possible[k]; !ok{
											matchBoth = false
										}else{
											pair.m_values[valIdx] = k
											valIdx++ 
										}
									}
									if matchBoth{
										foundPair.m_rowAligned = y2==y1
										if foundPair.m_rowAligned{
											foundPair.m_rowOrColNum = c1.m_y										
										}else{
											foundPair.m_rowOrColNum = c1.m_x										
										}
										foundPairs = append(foundPairs,foundPair)
									}
								}
							}
						}
					}
				}
			}
		}
	}*/
	
	return foundPairs,nil
}

//A horizontal or vertical line of 9 cells through the entire Grid.
type line struct {
	m_cells 		cellPtrSlice
	m_rowAligned	bool
	m_rowOrColNum	int
}

func(l *line) init(){
	l.m_cells = make([]*cell,COL_LENGTH,COL_LENGTH)
}

func (l line) Solved() (bool,error) {
	return l.m_cells.Solved()
}

func (l* line) reducePossible() (bool,error) {
	known,err := l.m_cells.Known()
	
	if err != nil {
		return false,err
	}
	
	reduced, err := l.m_cells.TakeKnownFromPossible(known)
	if err != nil{
		return false,err
	}
		
	return reduced,nil
}

func (l* line) validate() bool{
	known,err := l.m_cells.Known()
	if err != nil {
		return false
	}
	return validate(known)
}

func (l line) String() string{
	str := ""
	for _,c := range l.m_cells{
		if c.IsKnown(){
			v,err:= c.Known()
			if err == nil{
				str+=strconv.Itoa(v)	
			}
		}else{
			str+="x"
		}
	}
	return str
} 

//Grid which represents the 3x3 collection of squares which represent the entire puzzle
const ROW_LENGTH = 9
const COL_LENGTH = 9
const NUM_SQUARES = COL_LENGTH
const SQUARE_SIZE = 3

type Grid struct {
	m_squares 	[]square
	m_rows		[]line
	m_cols		[]line
	
	m_sets		[]Solver
	m_cells		[][]cell
}

func New(puzzle [COL_LENGTH][ROW_LENGTH]int) (*Grid, error){
	var g Grid
	g.Init();
	g.Fill(puzzle)
	return &g,nil
} 

func (g *Grid) validate() bool{
	for x,_ := range g.m_cells{
		for y,_:= range g.m_cells[x]{
			if !g.m_cells[x][y].validate(){
				return false
			}	
		}
	}
	
	for i,_ := range g.m_sets{
		if !g.m_sets[i].validate(){
			return false
		}
	}
	
	return true
}

func (g *Grid) Init() {
	//Init the raw cells themselves that actually store the grid data
	g.m_cells = make([][]cell,COL_LENGTH)
	for i :=0; i< len(g.m_cells); i++{
		g.m_cells[i] = make([]cell, ROW_LENGTH)
		
		for j :=0; j< len(g.m_cells[i]); j++{
			c := &g.m_cells[i][j]
			c.init(i,j)
		}
	} 
		
	//Init each of the grouping structures that view portions of the grid
	
	/*
	
	Squares are indexed into the grid as folows
	
	S0 S1 S2
	S3 S4 S5
	S6 S7 S8
	
	*/
	g.m_squares = make([]square,NUM_SQUARES)
	
	for squareIdx :=0; squareIdx<NUM_SQUARES; squareIdx++{
		
		g.m_squares[squareIdx].init()
		for x :=0; x<SQUARE_SIZE; x++{
			for y:= 0; y<SQUARE_SIZE; y++{
				//is this correct?
				gridX := SQUARE_SIZE * (squareIdx % SQUARE_SIZE) + x 
				gridY := SQUARE_SIZE * (squareIdx / SQUARE_SIZE) + y
				
				cellPtr := &g.m_cells[gridX][gridY]
				g.m_squares[squareIdx].m_cells[x][y] = cellPtr
			}
		}
		
	}
	
	g.m_rows = make([]line, ROW_LENGTH)
	g.m_cols = make([]line,COL_LENGTH)
	
	//Make m_sets just a big long list of all the cell grouping structures
	//handy for doing iterations over all different ways of looking at the cells
	g.m_sets = make([]Solver,len(g.m_squares) + len(g.m_rows) + len(g.m_cols))
	
	
	var idx int
	for i := 0; i<len(g.m_squares); i++{
		s:= &g.m_squares[i]
		g.m_sets[idx] = s
		idx++
	}
	
	
	for i:=0; i<len(g.m_rows); i++{
		r:=&g.m_rows[i]
		g.m_sets[idx] = r
		idx++
		r.init() 
		for colNum:=0; colNum < COL_LENGTH; colNum++{
			r.m_cells[colNum] = &g.m_cells[colNum][i]
		}
		r.m_rowAligned = true
		r.m_rowOrColNum = i
	}
	
	
	for i:= 0; i<len(g.m_cols); i++{
		c := &g.m_cols[i]
		g.m_sets[idx] = c
		idx++
		c.init() 
		for rowNum:=0; rowNum < ROW_LENGTH; rowNum++{
			c.m_cells[rowNum] = &g.m_cells[i][rowNum]
		}
		c.m_rowAligned = false
		c.m_rowOrColNum = i
	}
	
}

func (g *Grid) Fill(puzzle [COL_LENGTH][ROW_LENGTH]int){
	g.Init()
	
	
	for x:=0; x<COL_LENGTH; x++{
		for y:=0; y<ROW_LENGTH; y++{
			var puzzVal = puzzle[y][x]
			
			if puzzVal >=1 && puzzVal<=9{
				g.m_cells[x][y].SetKnownTo(puzzVal)
			}
		}
	}
	
}

func (g Grid) Solved() (bool,error) {
	for _,s := range g.m_sets{
		solved,err := s.Solved()
		if err != nil{
			fmt.Println("Error during Solved() check on grid: " + err.Error())
			return false,err
		}
		
		if !solved{
			return false,nil
		}
	}
	
	return true,nil
}

func (g *Grid) squareExclusionReduce() (bool,error){
	changed := false
	/*for i,_ := range g.m_squares{
		s := &g.m_squares[i]
		pairs, err := s.AlignedCellOnlyPairs()
		if err != nil{
			return false,err
		}
		for _,p := range pairs{
			
		}
		
	}*/
	
	return changed,nil	
}

func(g *Grid) reducePossiblePass() (bool, error){
	changed := false
	
	for pass:=0;pass<2;pass++{
		for i,_ := range g.m_sets{
			reduced,err := g.m_sets[i].reducePossible()
			if err != nil{
				return false,err
			}
			changed = changed || reduced
		}
		sqReduce,err := g.squareExclusionReduce()
		if err != nil{
				return false,err
		}
		changed = changed || sqReduce
	}
	
	return changed,nil
}

func (g *Grid) Puzzle() [COL_LENGTH][ROW_LENGTH]int{
	var puzzle [COL_LENGTH][ROW_LENGTH]int
	for x,_ := range puzzle{
		for y,_ := range puzzle[x]{
			if g.m_cells[x][y].IsKnown(){
				var err error
				puzzle[y][x],err = g.m_cells[x][y].Known()
				if err != nil{
					return puzzle
				}
			}
		}
	}
	
	return puzzle
}

func (g *Grid) setKnown( x,y, known int) error{
	//should probably check if grid is initialised and return error if it isn't
	g.m_cells[x][y].SetKnownTo(known)
	
	return nil
}

func (g *Grid) DuplicateGrid() (*Grid,error){
	return New(g.Puzzle())
}

func (g* Grid) TotalPossible() (int, error){
	totalPoss := 0
	for x,_ := range g.m_cells{
		for y,_:= range g.m_cells[x]{
			if !g.m_cells[x][y].IsKnown(){
				val,err := g.m_cells[x][y].Possibles()
				if err != nil{
					return 0,err
				}
				numPoss := len(val)
				totalPoss += numPoss
			}
		}
		
	}	
	return totalPoss,nil
}

func (g *Grid) GenerateGuessGrids() ([]*Grid, error){
	
	totalPoss,err := g.TotalPossible()
	guesses := make([]*Grid,0,totalPoss)
	if err != nil{
		return guesses,err
	}
	smallPoss := 99
	smX:=0
	smY:=0
	for x,_ := range g.m_cells{
		for y,_:= range g.m_cells[x]{
			if !g.m_cells[x][y].IsKnown(){
				
				possibles,err := g.m_cells[x][y].Possibles()
				if err!=nil{
					return guesses,err
				}
				if len(possibles) < smallPoss{
					smallPoss=len(possibles)
					smX=x
					smY=y
				}
				
			}
		}
	}
	
	if smallPoss < 99{
		possibles,err := g.m_cells[smX][smY].Possibles()
		if err!=nil{
			return guesses,err
		}
		for _,v := range possibles{
			guess,err := g.DuplicateGrid()
			if err!=nil{
				return guesses,err
			}
			err = guess.setKnown(smX,smY,v)
			if err!=nil{
				return guesses,err
			}
			guesses = append(guesses,guess)
		}
	}
		
	return guesses,nil
	
}

type SolveResult struct{
	m_grid 		*Grid
	m_solved 	bool
}

func (s *SolveResult) Grid() *Grid{
	return s.m_grid
}

func (s *SolveResult) Solved() bool{
	return s.m_solved
}

func startSolveRoutine(ch chan SolveResult, g *Grid) {
	
	defer close(ch)
	res, err := g.Solve()
	if err != nil{
		//this error might be expected, we might have sent in an invalid puzzle
		//only care about this response to print or pass on in the root call to solve.
		return
	}
	 
	ch<-*res
}

func (g *Grid) Solve() (*SolveResult,error){
	
	var err error
	for changed:=true; changed;{
		changed, err = g.reducePossiblePass()
		if err != nil{
			return &SolveResult{nil,false},err
		}
		if !g.validate(){
			return &SolveResult{nil,false},nil //this was probably an invalid guess, just want to stop trying to process this
		}
	}
	
	solved,err := g.Solved()
	if err != nil{
			return &SolveResult{nil,false},err
	}
	
	if solved{
		return &SolveResult{g,true},nil
	}
	
	guesses,err := g.GenerateGuessGrids()
	
	if err!=nil{
		return &SolveResult{g,false},err
	}
	
	resChans := make([]chan SolveResult,0,len(guesses))
	
	for _,guess := range guesses{
		ch := make(chan SolveResult)
		go startSolveRoutine(ch,guess)
		resChans = append(resChans,ch)
	}
	
	for i,_ := range resChans{
		for res:= range resChans[i] {
			if res.m_solved{
				return &res,nil
			}
		}
	}
	
	return &SolveResult{nil,false},errors.Wrap(SolveError{"Unable to solve puzzle"},1)
}

func (g *Grid) KnownEquals(puzzle [COL_LENGTH][ROW_LENGTH]int) bool{
	thisPuzz := g.Puzzle()
	for x,_ := range puzzle{
		for y,_ := range puzzle[x]{
			if puzzle[x][y] != thisPuzz[x][y]{
				return false
			}
		}
	}
	return true
} 

func (g Grid) String() string {
	var str string
	
	if len(g.m_cells) < COL_LENGTH{
		return "Grid probably not initialised"
	}
	
	for x:=0; x < ROW_LENGTH; x++{
		for y:=0; y < COL_LENGTH; y++{
			
			if len(g.m_cells[y]) < ROW_LENGTH{
				return "Error in grid not all rows correctly initialised"
			}
			
			cell := g.m_cells[y][x]
			str += cell.String()
		}
		str+="\n"
	}
	return str
}




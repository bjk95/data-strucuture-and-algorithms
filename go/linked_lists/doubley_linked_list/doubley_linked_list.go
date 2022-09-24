package doubleylinkedlist 

type LinkedListNode struct {
    Val int;
	Next *LinkedListNode;
	Prev *LinkedListNode
	}

type MyLinkedList struct {
	Head *LinkedListNode;
	Length int;
}


func Constructor() MyLinkedList {
	return MyLinkedList{
		Head: nil,
		Length: 0,
	}  
}


func (this *MyLinkedList) Get(index int) int {
	if index+1 > this.Length {
		return -1
	}
	node := this.Head
	for i := 0; i < index; i++ {
		node = node.Next
	}
	return node.Val
}


func (this *MyLinkedList) AddAtHead(val int)  {
	newHeadNode := &LinkedListNode{
		Val: val,
		Next: this.Head,
	}
	// newHeadNode.Next.Prev = newHeadNode
	this.Head = newHeadNode
	if this.Head.Next != nil {
		this.Head.Next.Prev = this.Head
	}
	this.Length++
}


func (this *MyLinkedList) AddAtTail(val int)  {
	newTailNode := LinkedListNode{
		Val: val,
		Next: nil,
	}

	last := this.Head
	if last == nil {
		this.Head = &newTailNode
		this.Length++
		return
	}

	for i := 0; i < this.Length; i++ {
		if last.Next == nil {
			newTailNode.Prev = last
			last.Next = &newTailNode
			this.Length++
			return
		} else {
			last = last.Next
		}
	}
	
    
}


func (this *MyLinkedList) AddAtIndex(index int, val int)  {
	if index > this.Length {
		return
	}

	last := this.Head
	if index == 0{
		this.AddAtHead(val)
	}

	for i := 1; i <= index; i++ {
		if i == index {
			newNode := LinkedListNode{
				Val: val,
			}
			if last != nil {
				newNode.Next = last.Next
				newNode.Prev = last
			}
			last.Next = &newNode
			this.Length++
			return
		} else {
			last = last.Next
		}
	} 
}


func (this *MyLinkedList) DeleteAtIndex(index int)  {
	if index + 1 > this.Length {
		return
	}

	last := this.Head
	if last == nil && index == 0{
		this.Head = nil
		this.Length = 0
		return
	}

	if index == 0 {
		this.Head = last.Next
		if this.Head != nil {
			this.Head.Prev = nil
		}
		this.Length--
		return
	}

	for i := 1; i <= index; i++ {
		if i == index {
			if last.Next != nil {
				last.Next = last.Next.Next
				if last.Next != nil {
					last.Next.Prev = last
				}
				
			} else {
				last.Next = nil
			}
			this.Length--
			return
		} else {
			last = last.Next
		}
	}  
}




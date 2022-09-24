package linkedlists

type ListNode struct {
    Val int;
	Next *ListNode;
	}

type MyLinkedList struct {
	Head *ListNode;
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
	newHeadNode := ListNode{
		Val: val,
		Next: this.Head,
	}
	this.Head = &newHeadNode
	this.Length++
}


func (this *MyLinkedList) AddAtTail(val int)  {
	newTailNode := ListNode{
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
		newHead := &ListNode{
			Val: val,
			}
		
		if last != nil {
			newHead.Next = last
		}
		this.Head = newHead
		this.Length++
		return
	}

	for i := 1; i <= index; i++ {
		if i == index {
			newNode := ListNode{
				Val: val,
			}
			if last != nil {
				newNode.Next = last.Next
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
		this.Length--
		return
	}

	for i := 1; i <= index; i++ {
		if i == index {
			if last.Next != nil {
				last.Next = last.Next.Next
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




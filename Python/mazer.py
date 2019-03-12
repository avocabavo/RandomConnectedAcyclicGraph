from argparse import ArgumentParser
import random

class Node:
	roster= []
	def __init__(self, group):
		self.group= group
		self.north= None
		self.south= None
		self.east= None
		self.west= None
		Node.roster.append(self)
	def symbol(self):
		if self.north and self.north.chosen:
			if self.south and self.south.chosen:
				if self.east and self.east.chosen:
					if self.west and self.west.chosen:
						return ('╋')
					else:
						return ('┣')
				else:
					if self.west and self.west.chosen:
						return ('┫')
					else:
						return ('┃')
			else:
				if self.east and self.east.chosen:
					if self.west and self.west.chosen:
						return ('┻')
					else:
						return ('┗')
				else:
					if self.west and self.west.chosen:
						return ('┛')
					else:
						return ('╹')
		else:
			if self.south and self.south.chosen:
				if self.east and self.east.chosen:
					if self.west and self.west.chosen:
						return ('┳')
					else:
						return ('┏')
				else:
					if self.west and self.west.chosen:
						return ('┓')
					else:
						return ('╻')
			else:
				if self.east and self.east.chosen:
					if self.west and self.west.chosen:
						return ('━')
					else:
						return ('╺')
				else:
					if self.west and self.west.chosen:
						return ('╸')
					else:
						return ('░')
	def symbols(self):
		if self.north and self.north.chosen:
			if self.south and self.south.chosen:
				if self.east and self.east.chosen:
					if self.west and self.west.chosen:
						return (' ┃ ','━╋━',' ┃ ')
					else:
						return (' ┃ ',' ┣━',' ┃ ')
				else:
					if self.west and self.west.chosen:
						return (' ┃ ','━┫ ',' ┃ ')
					else:
						return (' ┃ ',' ┃ ',' ┃ ')
			else:
				if self.east and self.east.chosen:
					if self.west and self.west.chosen:
						return (' ┃ ', '━┻━', '   ')
					else:
						return (' ┃ ', ' ┗━', '   ')
				else:
					if self.west and self.west.chosen:
						return (' ┃ ', '━┛ ', '   ')
					else:
						return (' ┃ ', ' ╹ ', '   ')
		else:
			if self.south and self.south.chosen:
				if self.east and self.east.chosen:
					if self.west and self.west.chosen:
						return ('   ', '━┳━', ' ┃ ')
					else:
						return ('   ', ' ┏━', ' ┃ ')
				else:
					if self.west and self.west.chosen:
						return ('   ', '━┓ ', ' ┃ ')
					else:
						return ('   ', ' ╻ ', ' ┃ ')
			else:
				if self.east and self.east.chosen:
					if self.west and self.west.chosen:
						return ('   ', '━━━', '   ')
					else:
						return ('   ', ' ╺━', '   ')
				else:
					if self.west and self.west.chosen:
						return ('   ', '━╸ ', '   ')
					else:
						return ('   ', ' ░ ', '   ')

class Segment:
	roster= []
	def __init__(self, one_end, other_end):
		self.one_end= one_end
		self.other_end= other_end
		self.chosen= False
		Segment.roster.append(self)
	def connect_if_not_already(self):
		if self.one_end.group != self.other_end.group:
			self.chosen= True
			new_group= self.one_end.group
			old_group= self.other_end.group
			for node in Node.roster:
				if node.group == old_group:
					node.group= new_group

def make_maze():
	parser= ArgumentParser(
		description= "Create a random tree connecting an n by n grid of nodes.")
	parser.add_argument(
		'n',
		help='Size of the node-grid.')
	args= parser.parse_args()
	n= int(args.n)

	a= []
	group_count= 0
	for i in range(n):
		new_row= []
		for j in range(n):
			star= Node(group_count)
			new_row.append(star)
			group_count += 1
		a.append(new_row)

	for i in range(n-1):
		for j in range(n):
			s= Segment(a[i][j], a[i+1][j])
			a[i][j].south= s
			a[i+1][j].north= s
	for i in range(n):
		for j in range(n-1):
			s= Segment(a[i][j], a[i][j+1])
			a[i][j].east= s
			a[i][j+1].west= s
	
	random.shuffle(Segment.roster)

	for seg in Segment.roster:
		seg.connect_if_not_already()

	if False:
		for i in range(n):
			current_row= []
			for j in range(n):
				current_row.append(a[i][j].symbols())
			for j in range(n):
				print(current_row[j][0], end='')
			print()
			for j in range(n):
				print(current_row[j][1], end='')
			print()
		# for j in range(n):
		# 	print(current_row[j][2], end='')
		# print()
	else:
		for i in range(n):
			for j in range(n):
				print(a[i][j].symbol(), end='')
			print()
	
	# print('node count: ' + str(len(Node.roster)))
	# print('segment count: ' + str(len(Segment.roster)))

if __name__ == '__main__':
	make_maze()

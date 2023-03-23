#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
StandmapHom.py
Created on Sun Aug  2  2020
"Introduction to Modern Dynamics" 2nd Edition (Oxford, 2019)
@author: nolte
"""
 
import numpy as np
from matplotlib import pyplot as plt
from numpy import linalg as LA
 
plt.close('all')
 
eps = 0.97
 
np.random.seed(2)
 
plt.figure(1)
 
for eloop in range(0,100):
 
    rlast = 2*np.pi*(0.5-np.random.random())
    thlast = 4*np.pi*np.random.random()
     
    rplot = np.zeros(shape=(200,))
    thetaplot = np.zeros(shape=(200,))
    for loop in range(0,200):
        rnew = rlast + eps*np.sin(thlast)
        thnew = np.mod(thlast+rnew,4*np.pi)
         
        thetaplot[loop] = np.mod(thnew-np.pi,4*np.pi)     
        rtemp = np.mod(rnew + np.pi,2*np.pi)
        rplot[loop] = rtemp - np.pi
   
        rlast = rnew
        thlast = thnew
         
    plt.plot(np.real(thetaplot),np.real(rplot),'o',ms=0.2)
    plt.xlim(xmin=np.pi,xmax=4*np.pi)
    plt.ylim(ymin=-2.5,ymax=2.5)
         
plt.savefig('StandMap')
 
K = eps
eps0 = 5e-7
 
J = [[1,1+K],[1,1]]
w, v = LA.eig(J)
 
My = w[0]
Vu = v[:,0]     # unstable manifold
Vs = v[:,1]     # stable manifold
 
# Plot the unstable manifold
Hr = np.zeros(shape=(100,150))
Ht = np.zeros(shape=(100,150))
for eloop in range(0,100):
     
    eps = eps0*eloop
 
    roldu1 = eps*Vu[0]
    thetoldu1 = eps*Vu[1]
     
    Nloop = np.ceil(-6*np.log(eps0)/np.log(eloop+2))
    flag = 1
    cnt = 0
     
    while flag==1 and cnt < Nloop:
         
        ru1 = roldu1 + K*np.sin(thetoldu1)
        thetau1 = thetoldu1 + ru1
         
        roldu1 = ru1
        thetoldu1 = thetau1
         
        if thetau1 > 4*np.pi:
            flag = 0
             
        Hr[eloop,cnt] = roldu1
        Ht[eloop,cnt] = thetoldu1 + 3*np.pi
        cnt = cnt+1
     
x = Ht[0:99,12] - 2*np.pi
x2 = 6*np.pi - x
y = Hr[0:99,12]
y2 = -y
plt.plot(x,y,linewidth =0.75)
plt.plot(x2,y2,linewidth =0.75)
 
del x,y
x = Ht[5:39,15] - 2*np.pi
x2 = 6*np.pi - x
y = Hr[5:39,15]
y2 = -y
plt.plot(x,y,linewidth =0.75)
plt.plot(x2,y2,linewidth =0.75)
 
del x,y
x = Ht[12:69,16] - 2*np.pi
x2 = 6*np.pi - x
y = Hr[12:69,16]
y2 = -y
plt.plot(x,y,linewidth =0.75)
plt.plot(x2,y2,linewidth =0.75)
 
del x,y
x = Ht[15:89,17] - 2*np.pi
x2 = 6*np.pi - x
y = Hr[15:89,17]
y2 = -y
plt.plot(x,y,linewidth =0.75)
plt.plot(x2,y2,linewidth =0.75)
 
del x,y
x = Ht[30:99,18] - 2*np.pi
x2 = 6*np.pi - x
y = Hr[30:99,18]
y2 = -y
plt.plot(x,y,linewidth =0.75)
plt.plot(x2,y2,linewidth =0.75)
 
# Plot the stable manifold
del Hr, Ht
Hr = np.zeros(shape=(100,150))
Ht = np.zeros(shape=(100,150))
#eps0 = 0.03
for eloop in range(0,100):
     
    eps = eps0*eloop
 
    roldu1 = eps*Vs[0]
    thetoldu1 = eps*Vs[1]
     
    Nloop = np.ceil(-6*np.log(eps0)/np.log(eloop+2))
    flag = 1
    cnt = 0
     
    while flag==1 and cnt < Nloop:
         
        thetau1 = thetoldu1 - roldu1
        ru1 = roldu1 - K*np.sin(thetau1)
 
        roldu1 = ru1
        thetoldu1 = thetau1
         
        if thetau1 > 4*np.pi:
            flag = 0
             
        Hr[eloop,cnt] = roldu1
        Ht[eloop,cnt] = thetoldu1
        cnt = cnt+1
     
x = Ht[0:79,12] + np.pi
x2 = 6*np.pi - x
y = Hr[0:79,12]
y2 = -y
plt.plot(x,y,linewidth =0.75)
plt.plot(x2,y2,linewidth =0.75)
 
del x,y
x = Ht[4:39,15] + np.pi
x2 = 6*np.pi - x
y = Hr[4:39,15]
y2 = -y
plt.plot(x,y,linewidth =0.75)
plt.plot(x2,y2,linewidth =0.75)
 
del x,y
x = Ht[12:69,16] + np.pi
x2 =  6*np.pi - x
y = Hr[12:69,16]
y2 = -y
plt.plot(x,y,linewidth =0.75)
plt.plot(x2,y2,linewidth =0.75)
 
del x,y
x = Ht[15:89,17] + np.pi
x2 =  6*np.pi - x
y = Hr[15:89,17]
y2 = -y
plt.plot(x,y,linewidth =0.75)
plt.plot(x2,y2,linewidth =0.75)
 
del x,y
x = Ht[30:99,18] + np.pi
x2 =  6*np.pi - x
y = Hr[30:99,18]
y2 = -y
plt.plot(x,y,linewidth =0.75)
plt.plot(x2,y2,linewidth =0.75)